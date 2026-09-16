#!/usr/bin/env python3
"""Exercise the built binary in real Unix PTYs, without third-party packages.

Run `cargo build --locked`, then `python3 tests/terminal_e2e.py`.
The two natural one-minute completion checks run concurrently with the quick
checks, so the entire suite normally takes about 63 seconds. All persisted
files live under temporary homes. Use `--binary PATH` to select another binary.
"""

import argparse
import codecs
from contextlib import ExitStack
import fcntl
import json
import os
from pathlib import Path
import pty
import re
import select
import signal
import struct
import subprocess
import sys
import tempfile
import termios
import time


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", type=Path, default=Path("target/debug/tomatui"))
BINARY = parser.parse_args().binary.resolve()


class Screen:
    """The cursor/erase subset emitted by crossterm, with incremental decoding.

    Reconstructing cells lets assertions see the current screen, including
    ratatui's differential updates, instead of matching stale transcript text.
    """

    def __init__(self, cols, rows):
        self.cols, self.rows = cols, rows
        self.cells = {}
        self.x = self.y = 0
        self.pending = ""
        self.decoder = codecs.getincrementaldecoder("utf-8")("replace")
        self.hidden = False
        self.alternate = False

    def feed(self, data):
        self.pending += self.decoder.decode(data)
        while self.pending:
            if self.pending.startswith("\x1b"):
                match = re.match(r"\x1b\[([0-?]*)([ -/]*)([@-~])", self.pending)
                if not match:
                    # All escapes emitted by this application are CSI.
                    if len(self.pending) == 1 or self.pending.startswith("\x1b["):
                        return
                    raise AssertionError(f"Unsupported terminal escape: {self.pending[:30]!r}")
                self.pending = self.pending[match.end():]
                args, _, command = match.groups()
                values = [int(v or 0) for v in args.lstrip("?").split(";")]
                n = values[0] or 1
                if command in "Hf":
                    self.y = n - 1
                    self.x = (values[1] or 1) - 1 if len(values) > 1 else 0
                elif command == "G":
                    self.x = n - 1
                elif command in "ABCD":
                    dx, dy = {"A": (0, -n), "B": (0, n), "C": (n, 0), "D": (-n, 0)}[command]
                    self.x, self.y = max(0, self.x + dx), max(0, self.y + dy)
                elif command == "J":
                    if values[0] in (2, 3):
                        self.cells.clear()
                    else:
                        self.cells = {p: c for p, c in self.cells.items() if p < (self.y, self.x)}
                elif command == "K":
                    mode = values[0]
                    self.cells = {p: c for p, c in self.cells.items()
                                  if p[0] != self.y or (mode == 0 and p[1] < self.x)
                                  or (mode == 1 and p[1] > self.x)}
                elif args.startswith("?") and command in "hl":
                    if 25 in values:
                        self.hidden = command == "l"
                    if 1049 in values:
                        self.alternate = command == "h"
                        self.cells.clear()
                continue
            char, self.pending = self.pending[0], self.pending[1:]
            if char == "\r":
                self.x = 0
            elif char == "\n":
                self.y += 1
            elif char == "\b":
                self.x = max(0, self.x - 1)
            elif char >= " ":
                self.cells[self.y, self.x] = char
                self.x += 1

    def text(self):
        return "\n".join("".join(self.cells.get((y, x), " ") for x in range(self.cols)).rstrip()
                         for y in range(max(self.rows, self.y + 1)))


class Home:
    def __enter__(self):
        self.temp = tempfile.TemporaryDirectory(prefix="tomatui-e2e-")
        self.root = Path(self.temp.name)
        bin_dir = self.root / "bin"
        bin_dir.mkdir()
        for name in ("notify-send", "osascript"):
            stub = bin_dir / name
            stub.write_text("#!/bin/sh\nexit 0\n")
            stub.chmod(0o755)
        self.env = dict(os.environ, HOME=str(self.root),
                        XDG_CONFIG_HOME=str(self.root / "config"),
                        XDG_DATA_HOME=str(self.root / "data"),
                        XDG_CACHE_HOME=str(self.root / "cache"),
                        TERM="xterm-256color", NO_COLOR="1",
                        PATH=str(bin_dir) + os.pathsep + os.environ.get("PATH", ""))
        base = self.root / "Library/Application Support" if sys.platform == "darwin" else None
        self.config = (base or self.root / "config") / "tomatui/config.json"
        self.stats = (base or self.root / "data") / "tomatui/stats.json"
        return self

    def __exit__(self, *_):
        self.temp.cleanup()

    def cli(self, *args):
        result = subprocess.run([str(BINARY), *args], env=self.env, capture_output=True,
                                text=True, timeout=10)
        assert result.returncode == 0, result.stderr
        return result.stdout

    def totals(self):
        if not self.stats.exists():
            return (0, 0)
        days = json.loads(self.stats.read_text())["days"].values()
        return tuple(sum(day[key] for day in days) for key in ("pomodoros", "work_minutes"))


class Terminal:
    active = []

    def __init__(self, home, *args, cols=80, rows=24):
        self.home, self.args = home, args
        self.master, self.slave = pty.openpty()
        self.original = termios.tcgetattr(self.slave)
        self.screen = Screen(cols, rows)
        self.transcript = bytearray()
        self.resize(cols, rows)

        # Keep the PTY open in the parent to inspect termios after exit. A new
        # session avoids inheriting the developer's controlling terminal;
        # crossterm uses these TTY stdin/stdout descriptors directly.
        self.process = subprocess.Popen([str(BINARY), *args], env=home.env,
                                        stdin=self.slave, stdout=self.slave, stderr=self.slave,
                                        start_new_session=True)
        self.active.append(self)

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.active.remove(self)
        if self.process.poll() is None:
            self.process.kill()
        # Closing the PTY also releases a child blocked in terminal output on macOS.
        os.close(self.master)
        os.close(self.slave)
        self.process.wait(timeout=5)

    def resize(self, cols, rows):
        fcntl.ioctl(self.slave, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))
        self.screen.cols, self.screen.rows = cols, rows
        if hasattr(self, "process"):
            os.kill(self.process.pid, signal.SIGWINCH)

    def read(self, timeout=0.05):
        # Drain every live PTY so concurrent completion checks cannot block
        # on terminal output while another test is waiting for input.
        terminals = {terminal.master: terminal for terminal in self.active}
        for fd in select.select(list(terminals), [], [], timeout)[0]:
            data = os.read(fd, 65536)
            terminal = terminals[fd]
            terminal.transcript.extend(data)
            terminal.screen.feed(data)

    def until(self, predicate, description, timeout=5):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            self.read()
            if predicate():
                return
        raise AssertionError(f"Timed out: {description}; args={self.args!r}\n"
                             f"Screen:\n{self.screen.text()}\n"
                             f"Transcript tail: {bytes(self.transcript[-4000:])!r}")

    def expect(self, text, timeout=5):
        self.until(lambda: text in self.screen.text(), repr(text), timeout)

    def send(self, keys):
        os.write(self.master, keys.encode())

    def settle(self, seconds):
        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline:
            self.read()

    def clean_exit(self, key=None):
        if key:
            self.send(key)
        self.until(lambda: self.process.poll() is not None, "process exit")
        self.settle(0.1)
        assert self.process.returncode == 0, bytes(self.transcript[-2000:])
        assert termios.tcgetattr(self.slave) == self.original, "terminal attributes not restored"
        assert not self.screen.hidden, "cursor remains hidden"
        assert not self.screen.alternate, "alternate screen remains active"
        assert "Completed" in self.screen.text(), "missing session summary"


def quick_checks():
    with Home() as home:
        home.cli("config", "-w", "2m", "-b", "3m", "--on-end", "ask")
        assert home.config.exists(), "configuration escaped the temporary home"
        saved = home.config.read_bytes()
        assert json.loads(saved)["work_minutes"] == 2
        assert "2" in home.cli("config")
        with Terminal(home) as terminal:
            terminal.expect("Work")
            assert terminal.screen.alternate and terminal.screen.hidden
            terminal.expect("Session 1 / 4")
            terminal.send("pw")
            terminal.expect("paused")
            terminal.resize(32, 6)
            terminal.expect("02:00  Work  paused")
            terminal.settle(1.2)
            terminal.expect("02:00  Work  paused")
            terminal.send("+")
            terminal.expect("03:00  Work  paused")
            terminal.send("p")
            terminal.until(lambda: "paused" not in terminal.screen.text(), "resumed timer")
            terminal.send("pb")
            terminal.expect("03:00  Break  paused")
            terminal.send("w")
            terminal.expect("02:00  Work")
            terminal.send("s")
            terminal.expect("Break")
            terminal.resize(1, 1)
            terminal.settle(0.2)
            terminal.resize(80, 24)
            terminal.expect("Session 2 / 4")
            terminal.expect("Break")
            terminal.clean_exit("q")
        assert home.totals() == (0, 0), "skipped work counted as complete"
        with Terminal(home, "1h", "4m", "-m") as terminal:
            terminal.expect("Work")
            assert terminal.screen.hidden and not terminal.screen.alternate
            terminal.send("pw")
            terminal.expect("60:00 Work paused")
            terminal.send("+")
            terminal.expect("61:00 Work paused")
            terminal.settle(0.1)
            output_size = len(terminal.transcript)
            terminal.send("z")
            terminal.settle(0.3)
            assert len(terminal.transcript) == output_size, "ignored key redrew a paused timer"
            terminal.send(" ")
            terminal.until(lambda: "paused" not in terminal.screen.text(), "space resumes")
            terminal.expect("60:59 Work", timeout=4)
            terminal.send("pb")
            terminal.expect("04:00 Break paused")
            terminal.resize(12, 2)
            terminal.expect("04:00 Break")
            terminal.resize(1, 1)
            terminal.settle(0.2)
            terminal.resize(80, 24)
            terminal.expect("p pause")
            terminal.clean_exit("\x03")
        assert home.config.read_bytes() == saved, "one-run overrides changed saved settings"
        assert home.totals() == (0, 0)
        # Both exit keys must restore both terminal modes.
        for args, key in [((), "\x03"), (("start", "-m", "-s", "1", "-l", "2m"), "q")]:
            with Terminal(home, *args) as terminal:
                terminal.expect("Work")
                if args:
                    terminal.send("pb")
                    terminal.expect("02:00 Long Break paused")
                terminal.clean_exit(key)
        with Terminal(home, "2m", "-m") as terminal:
            terminal.expect("Work")
            terminal.send("pw")
            terminal.expect("02:00 Work paused")
            for _ in range(24):
                terminal.send("p")
                terminal.until(lambda: "paused" not in terminal.screen.text(), "rapid resume")
                terminal.settle(0.05)
                terminal.send("p")
                terminal.expect("paused")
            assert "02:00 Work" not in terminal.screen.text(), "rapid pauses discarded elapsed time"
            terminal.clean_exit("q")
    print("PASS config, positional CLI, both modes, controls, resize, skip and cleanup", flush=True)


def main():
    if sys.platform not in ("linux", "darwin"):
        raise SystemExit("PTY checks require Linux or macOS")
    if not BINARY.is_file():
        raise SystemExit(f"Build the binary first: {BINARY}")
    started = time.monotonic()
    with ExitStack() as stack:
        ask_home = stack.enter_context(Home())
        quit_home = stack.enter_context(Home())
        ask = stack.enter_context(Terminal(ask_home, "1m", "-m", "--on-end", "ask"))
        quit_timer = stack.enter_context(Terminal(quit_home, "1m", "-m", "--on-end", "quit"))
        ask.expect("Work")
        quit_timer.expect("Work")
        quick_checks()
        ask.expect("00:00 Work complete", timeout=70)
        assert ask_home.totals() == (1, 1), "completion not persisted exactly once"
        ask.settle(1.2)
        ask.expect("00:00 Work complete")
        assert ask_home.totals() == (1, 1), "waiting recorded duplicate completion"
        ask.send("\r")
        ask.expect("Break")
        assert "complete" not in ask.screen.text(), "Enter did not continue the timer"
        ask.clean_exit("q")
        assert ask_home.totals() == (1, 1)
        quit_timer.clean_exit()
        assert quit_home.totals() == (1, 1), "quit completion not persisted exactly once"
    print(f"PASS natural completion, ask/Enter, quit and persistence ({time.monotonic() - started:.1f}s)")


if __name__ == "__main__":
    main()
