use chrono::*;
use slack_hook::{PayloadBuilder, Slack};

const WORKING_HOURS: i64 = 8;
const LUNCH_HOURS: i64 = 1;

// Environments keys
const SLACK_WEB_HOOK_URL_KEY: &str = "SLACK_WEB_HOOK_URL";
const SLACK_CHANNEL_KEY: &str = "SLACK_CHANNEL";
const SLACK_USER_NAME_KEY: &str = "SLACK_USER_NAME";
const SLACK_ICON_EMOJI_KEY: &str = "SLACK_ICON_EMOJI";

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
    let end_of_work_time = now + Duration::hours(LUNCH_HOURS);
    let end_of_work_time_str = end_of_work_time.format("%H:%M:%S").to_string();

    format!("🍱 休憩します。 ({}-{})", now_str, end_of_work_time_str)
}

pub fn evening() -> String {
    "♨️  お疲れさまでした。".to_string()
}

pub fn post_to_slack(message: String) {
    let web_hook_url = dotenv::var(SLACK_WEB_HOOK_URL_KEY).unwrap();
    let channel = dotenv::var(SLACK_CHANNEL_KEY).unwrap();
    let user_name = dotenv::var(SLACK_USER_NAME_KEY).unwrap();
    let icon_emoji = dotenv::var(SLACK_ICON_EMOJI_KEY).unwrap();

    let slack = Slack::new(web_hook_url.as_str());
    let payload = PayloadBuilder::new()
        .text(message)
        .channel(channel.as_str())
        .username(user_name.as_str())
        .icon_emoji(icon_emoji.as_str())
        .build()
        .unwrap();

    if let Ok(ref s) = slack {
        let _res = s.send(&payload);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morning() {
        let now = Local.with_ymd_and_hms(2022, 2, 28, 9, 10, 28).unwrap();
        assert_eq!(morning(now), "🏢 おはようございます。 (09:10:28-18:10:28)");
    }

    #[test]
    fn test_lunch() {
        let now = Local.with_ymd_and_hms(2022, 2, 28, 12, 12, 23).unwrap();
        assert_eq!(lunch(now), "🍱 休憩します。 (12:12:23-13:12:23)");
    }

    #[test]
    fn test_evening() {
        assert_eq!(evening(), "♨️  お疲れさまでした。");
    }
}
