use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::constants::{APP_NAME, DATE_FORMAT};

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
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME);
    fs::create_dir_all(&data_dir).ok();
    data_dir.join("stats.json")
}

pub fn load_stats() -> Stats {
    let path = stats_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Stats::default(),
    }
}

pub fn save_stats(stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
    let path = stats_path();
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn record_pomodoro(work_minutes: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut stats = load_stats();
    let today = Local::now().format(DATE_FORMAT).to_string();
    stats.record(&today, work_minutes);
    save_stats(&stats)
}

pub fn print_today() {
    let stats = load_stats();
    let today = Local::now().format(DATE_FORMAT).to_string();
    let (pomos, minutes) = stats.get_day(&today);
    let h = minutes / 60;
    let m = minutes % 60;
    println!("\n  Today ({})", today);

    println!("  {}", "-".repeat(30));
    println!("  Pomodoros:   {}", pomos);
    println!("  Work time:   {}h {}m", h, m);
    println!();
}

pub fn print_summary() {
    let stats = load_stats();
    let (total_pomos, total_minutes) = stats.total();
    let h = total_minutes / 60;
    let m = total_minutes % 60;
    let days_active = stats.days.len();
    let avg = if days_active > 0 {
        total_pomos as f64 / days_active as f64
    } else {
        0.0
    };

    println!("\n  All-time Summary");
    println!("  {}", "-".repeat(30));
    println!("  Total pomodoros:  {}", total_pomos);
    println!("  Total work time:  {}h {}m", h, m);
    println!("  Active days:      {}", days_active);
    println!("  Avg per day:      {:.1}", avg);
    println!();
}

pub fn print_history(days: u32) {
    let stats = load_stats();
    let now = Local::now();

    println!("\n  Pomodoro History (last {} days)", days);
    println!("  {}", "-".repeat(44));

    let mut period_pomos = 0u32;
    let mut period_minutes = 0u64;

    for i in 0..days {
        let date = (now - chrono::Duration::days(i as i64))
            .format(DATE_FORMAT)
            .to_string();
        let label = if i == 0 {
            "Today".to_string()
        } else if i == 1 {
            "Yesterday".to_string()
        } else {
            date.clone()
        };

        if let Some(day) = stats.days.get(&date) {
            let hours = day.work_minutes / 60;
            let mins = day.work_minutes % 60;
            let bar = "\u{2588}".repeat(day.pomodoros as usize);
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

    println!("  {}", "-".repeat(44));
    let total_h = period_minutes / 60;
    let total_m = period_minutes % 60;
    println!(
        "  Period total: {} pomodoros, {}h {}m\n",
        period_pomos, total_h, total_m
    );
}

pub fn clear_stats() {
    if let Err(e) = save_stats(&Stats::default()) {
        eprintln!("Failed to clear stats: {}", e);
        return;
    }
    println!("  All statistics cleared.");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
