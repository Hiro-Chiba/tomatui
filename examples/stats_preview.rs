//! Render fictional data through the same history renderer used by `tomatui stats`.
//! This example never reads or writes personal statistics.
#[allow(dead_code)]
#[path = "../src/constants.rs"]
mod constants;
#[allow(dead_code)]
#[path = "../src/stats.rs"]
mod stats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let today = chrono::NaiveDate::from_ymd_opt(2026, 9, 16).unwrap();
    let mut history = stats::Stats::default();
    for (offset, count) in [4, 6, 3, 0, 5, 2, 4].into_iter().enumerate() {
        let date = (today - chrono::Days::new(offset as u64))
            .format("%Y-%m-%d")
            .to_string();
        for _ in 0..count {
            history.record(&date, 25);
        }
    }
    println!("  Sample week · fictional data");
    stats::print_history_for(&history, today, 7)
}
