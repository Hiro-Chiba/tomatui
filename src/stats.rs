use chrono::{Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::{APP_NAME, DATE_FORMAT, FULL_BLOCK, MINUTES_PER_HOUR};

const STATS_FILE_NAME: &str = "stats.json";
const SUMMARY_SEPARATOR_WIDTH: usize = 30;
const HISTORY_SEPARATOR_WIDTH: usize = 44;
const MAX_HISTORY_BAR_WIDTH: u32 = 30;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct DayStat {
    pub pomodoros: u32,
    pub work_minutes: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Stats {
    pub days: BTreeMap<String, DayStat>,
}

impl Stats {
    pub fn record(&mut self, date: &str, work_minutes: u64) {
        let day = self.days.entry(date.to_string()).or_default();
        day.pomodoros += 1;
        day.work_minutes += work_minutes;
    }

    pub fn get_day(&self, date: &str) -> (u32, u64) {
        match self.days.get(date) {
            Some(day) => (day.pomodoros, day.work_minutes),
            None => (0, 0),
        }
    }

    pub fn total(&self) -> (u32, u64) {
        let mut pomos = 0u32;
        let mut mins = 0u64;
        for day in self.days.values() {
            pomos += day.pomodoros;
            mins += day.work_minutes;
        }
        (pomos, mins)
    }
}

fn stats_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME)
        .join(STATS_FILE_NAME)
}

fn load_stats_from(path: &Path) -> Result<Stats, Box<dyn Error>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(serde_json::from_str(&content)?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Stats::default()),
        Err(error) => Err(error.into()),
    }
}

fn save_stats_to(path: &Path, stats: &Stats) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(path, json)?;
    Ok(())
}

fn record_pomodoro_at(
    path: &Path,
    date: &str,
    work_minutes: u64,
) -> Result<(u32, u64), Box<dyn Error>> {
    let mut stats = load_stats_from(path)?;
    stats.record(date, work_minutes);
    let day = stats.get_day(date);
    save_stats_to(path, &stats)?;
    Ok(day)
}

pub fn load_stats() -> Result<Stats, Box<dyn Error>> {
    load_stats_from(&stats_path())
}

pub fn record_pomodoro(work_minutes: u64) -> Result<(u32, u64), Box<dyn Error>> {
    let today = Local::now().format(DATE_FORMAT).to_string();
    record_pomodoro_at(&stats_path(), &today, work_minutes)
}

pub fn print_today() -> Result<(), Box<dyn Error>> {
    let stats = load_stats()?;
    let today = Local::now().format(DATE_FORMAT).to_string();
    let (pomos, minutes) = stats.get_day(&today);
    let h = minutes / MINUTES_PER_HOUR;
    let m = minutes % MINUTES_PER_HOUR;
    println!("\n  Today ({})", today);

    println!("  {}", "-".repeat(SUMMARY_SEPARATOR_WIDTH));
    println!("  Pomodoros:   {}", pomos);
    println!("  Work time:   {}h {}m", h, m);
    println!();
    Ok(())
}

pub fn print_summary() -> Result<(), Box<dyn Error>> {
    let stats = load_stats()?;
    let (total_pomos, total_minutes) = stats.total();
    let h = total_minutes / MINUTES_PER_HOUR;
    let m = total_minutes % MINUTES_PER_HOUR;
    let days_active = stats.days.len();
    let avg = if days_active > 0 {
        total_pomos as f64 / days_active as f64
    } else {
        0.0
    };

    println!("\n  All-time Summary");
    println!("  {}", "-".repeat(SUMMARY_SEPARATOR_WIDTH));
    println!("  Total pomodoros:  {}", total_pomos);
    println!("  Total work time:  {}h {}m", h, m);
    println!("  Active days:      {}", days_active);
    println!("  Avg per day:      {:.1}", avg);
    println!();
    Ok(())
}

pub fn print_history(days: u32) -> Result<(), Box<dyn Error>> {
    let stats = load_stats()?;
    let today = Local::now().date_naive();

    println!("\n  Pomodoro History (last {} days)", days);
    println!("  {}", "-".repeat(HISTORY_SEPARATOR_WIDTH));

    let mut period_pomos = 0u32;
    let mut period_minutes = 0u64;

    for i in 0..days {
        let date = history_date(today, i)?.format(DATE_FORMAT).to_string();
        let label = if i == 0 {
            "Today".to_string()
        } else if i == 1 {
            "Yesterday".to_string()
        } else {
            date.clone()
        };

        if let Some(day) = stats.days.get(&date) {
            let hours = day.work_minutes / MINUTES_PER_HOUR;
            let mins = day.work_minutes % MINUTES_PER_HOUR;
            let bar = FULL_BLOCK.repeat(day.pomodoros.min(MAX_HISTORY_BAR_WIDTH) as usize);
            println!(
                "  {:<12} {:>2} pomos  {:>2}h {:>2}m  {}",
                label, day.pomodoros, hours, mins, bar
            );
            period_pomos += day.pomodoros;
            period_minutes += day.work_minutes;
        } else {
            println!("  {:<12}  0 pomos   0h  0m", label);
        }
    }

    println!("  {}", "-".repeat(HISTORY_SEPARATOR_WIDTH));
    let total_h = period_minutes / MINUTES_PER_HOUR;
    let total_m = period_minutes % MINUTES_PER_HOUR;
    println!(
        "  Period total: {} pomodoros, {}h {}m\n",
        period_pomos, total_h, total_m
    );
    Ok(())
}

fn history_date(today: NaiveDate, days_ago: u32) -> io::Result<NaiveDate> {
    today
        .checked_sub_days(Days::new(u64::from(days_ago)))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "history range is too large"))
}

pub fn clear_stats() -> Result<(), Box<dyn Error>> {
    save_stats_to(&stats_path(), &Stats::default())?;
    println!("  All statistics cleared.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            Self(
                std::env::temp_dir()
                    .join(format!("tomatui-stats-test-{}-{id}", std::process::id())),
            )
        }

        fn file(&self) -> PathBuf {
            self.0.join("stats.json")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn test_record_increments_pomodoros() {
        let mut stats = Stats::default();
        stats.record("2026-03-07", 25);
        assert_eq!(stats.get_day("2026-03-07"), (1, 25));

        stats.record("2026-03-07", 25);
        assert_eq!(stats.get_day("2026-03-07"), (2, 50));
    }

    #[test]
    fn test_different_days_are_independent() {
        let mut stats = Stats::default();
        stats.record("2026-03-07", 25);
        stats.record("2026-03-08", 25);

        assert_eq!(stats.get_day("2026-03-07"), (1, 25));
        assert_eq!(stats.get_day("2026-03-08"), (1, 25));
    }

    #[test]
    fn test_nonexistent_day_returns_zero() {
        let stats = Stats::default();
        assert_eq!(stats.get_day("2099-01-01"), (0, 0));
    }

    #[test]
    fn test_total_across_days() {
        let mut stats = Stats::default();
        stats.record("2026-03-07", 25);
        stats.record("2026-03-07", 25);
        stats.record("2026-03-08", 30);

        let (pomos, mins) = stats.total();
        assert_eq!(pomos, 3);
        assert_eq!(mins, 80);
    }

    #[test]
    fn test_today_resets_for_new_date() {
        let mut stats = Stats::default();
        stats.record("2026-03-07", 25);
        stats.record("2026-03-07", 25);

        // Next day has no records
        assert_eq!(stats.get_day("2026-03-08"), (0, 0));

        // Original day still intact
        assert_eq!(stats.get_day("2026-03-07"), (2, 50));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut stats = Stats::default();
        stats.record("2026-03-07", 25);
        stats.record("2026-03-08", 30);

        let json = serde_json::to_string(&stats).unwrap();
        let loaded: Stats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats, loaded);
    }

    #[test]
    fn test_empty_stats_total() {
        let stats = Stats::default();
        assert_eq!(stats.total(), (0, 0));
    }

    #[test]
    fn missing_file_is_empty_without_creating_directories() {
        let dir = TestDir::new();
        assert_eq!(load_stats_from(&dir.file()).unwrap(), Stats::default());
        assert!(!dir.0.exists());
    }

    #[test]
    fn record_persists_to_an_isolated_path() {
        let dir = TestDir::new();
        assert_eq!(
            record_pomodoro_at(&dir.file(), "2026-08-30", 25).unwrap(),
            (1, 25)
        );
        assert_eq!(
            record_pomodoro_at(&dir.file(), "2026-08-30", 30).unwrap(),
            (2, 55)
        );
        assert_eq!(
            load_stats_from(&dir.file()).unwrap().get_day("2026-08-30"),
            (2, 55)
        );
    }

    #[test]
    fn corrupt_stats_are_not_overwritten() {
        let dir = TestDir::new();
        fs::create_dir_all(&dir.0).unwrap();
        let original = b"not json";
        fs::write(dir.file(), original).unwrap();

        assert!(record_pomodoro_at(&dir.file(), "2026-08-30", 25).is_err());
        assert_eq!(fs::read(dir.file()).unwrap(), original);
    }

    #[test]
    fn history_dates_use_calendar_days() {
        let march_first = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        assert_eq!(
            history_date(march_first, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
    }
}
