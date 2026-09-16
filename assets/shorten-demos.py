"""Cut the recorded sessions into short README loops without changing app behavior."""
from pathlib import Path
import json
import subprocess

ROOT = Path(__file__).resolve().parent.parent
# Source start, source end, playback multiplier. Keep the command under one second.
CUTS = {
    "demo": [
        (0, 2.25, 2.5),
        (2.25, 8.9, 6.65),
        (9.2, 9.9, 1),
        (11.3, 12, 1),
        (12.3, 26.95, 12.2),
        (27.3, 28, 1),
        (28.3, 28.85, 1),
        (29.25, 29.95, 0.28),
    ],
    "minimal": [
        (0, 2.2, 2.5),
        (2.2, 9.7, 6),
        (10.1, 11.1, 1.5),
        (12.3, 16.4, 4.1),
    ],
}

for name, cuts in CUTS.items():
    source = ROOT / "target" / "recordings" / f"{name}.mp4"
    video = ROOT / "assets" / f"{name}.mp4"
    gif = video.with_suffix(".gif")
    info = json.loads(subprocess.check_output([
        "ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "json", str(source),
    ]))
    if float(info["format"]["duration"]) < cuts[-1][1]:
        raise SystemExit(f"{source} ends before the final cut. Re-record or adjust the cut times.")
    filters = [
        f"[0:v]trim=start={start}:end={end},setpts=(PTS-STARTPTS)/{speed},fps=30[v{i}]"
        for i, (start, end, speed) in enumerate(cuts)
    ]
    filters.append("".join(f"[v{i}]" for i in range(len(cuts))) + f"concat=n={len(cuts)}:v=1:a=0[out]")
    subprocess.run([
        "ffmpeg", "-y", "-i", str(source), "-filter_complex", ";".join(filters),
        "-map", "[out]", "-an", "-c:v", "libx264", "-crf", "18", "-pix_fmt", "yuv420p",
        "-movflags", "+faststart", str(video), "-loglevel", "error",
    ], check=True)
    subprocess.run([
        "ffmpeg", "-y", "-i", str(video), "-filter_complex",
        "fps=20,split[a][b];[a]palettegen[p];[b][p]paletteuse=dither=bayer:bayer_scale=3",
        "-loop", "0", str(gif), "-loglevel", "error",
    ], check=True)
    print(f"Updated {video.name} and {gif.name}", flush=True)
