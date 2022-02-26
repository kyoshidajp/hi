use chrono::{DateTime, Duration, Local, TimeZone};

const WORKING_HOURS: i64 = 8;
const LUNCH_HOURS: i64 = 1;

pub fn morning(now: DateTime<Local>) -> String {
    let now_str = now.format("%H:%M:%S").to_string();
    let end_of_work_time = now + Duration::hours(WORKING_HOURS + LUNCH_HOURS);
    let end_of_work_time_str = end_of_work_time.format("%H:%M:%S").to_string();

    format!(
        "🏢 おはようございます。 ({}-{})",
        now_str, end_of_work_time_str
    )
}

pub fn lunch(now: DateTime<Local>) -> String {
    let now_str = now.format("%H:%M:%S").to_string();
    let end_of_work_time = now + Duration::hours(1);
    let end_of_work_time_str = end_of_work_time.format("%H:%M:%S").to_string();

    format!("🍱 休憩します。 ({}-{})", now_str, end_of_work_time_str)
}

pub fn evening() -> String {
    "♨️  お疲れさまでした。".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morning() {
        let now = Local.ymd(2022, 2, 28).and_hms(9, 10, 28);
        assert_eq!(morning(now), "🏢 おはようございます。 (09:10:28-18:10:28)");
    }

    #[test]
    fn test_lunch() {
        let now = Local.ymd(2022, 2, 28).and_hms(12, 12, 23);
        assert_eq!(lunch(now), "🍱 休憩します。 (12:12:23-13:12:23)");
    }

    #[test]
    fn test_evening() {
        assert_eq!(evening(), "♨️  お疲れさまでした。");
    }
}
