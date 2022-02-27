mod hi;

use argh::FromArgs;
use chrono::{DateTime, Local};
use dotenv::dotenv;

#[derive(FromArgs)]
/// Arguments
struct Arg {
    /// an optional time slot
    #[argh(option, short = 't', default = "default_time_slot()")]
    time_slot: String,

    /// whether or not to post to slack (default: No)
    #[argh(switch)]
    post_to_slack: bool,
}

fn default_time_slot() -> String {
    "morning".to_string()
}

fn main() {
    let arg: Arg = argh::from_env();
    let slot = arg.time_slot;
    let post_to_slack = arg.post_to_slack;

    dotenv().ok();

    do_hi(slot, post_to_slack);
}

fn do_hi(slot: String, post_to_slack: bool) {
    let now: DateTime<Local> = Local::now();
    let message = match &*slot {
        "morning" => hi::morning(now),
        "lunch" => hi::lunch(now),
        "evening" => hi::evening(),
        _ => hi::morning(now),
    };

    println!("{}", message);

    if post_to_slack {
        hi::post_to_slack(message);
    }
}
