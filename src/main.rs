mod hi;

use argh::FromArgs;
use chrono::{DateTime, Local};

#[derive(FromArgs)]
/// Arguments
struct Arg {
    /// an optional time slot
    #[argh(option, short = 't', default = "default_time_slot()")]
    time_slot: String,
}

fn default_time_slot() -> String {
    "morning".to_string()
}

fn main() {
    let arg: Arg = argh::from_env();
    let slot = arg.time_slot;

    do_hi(slot);
}

fn do_hi(slot: String) {
    let now: DateTime<Local> = Local::now();
    let message = match &*slot {
        "morning" => hi::morning(now),
        "lunch" => hi::lunch(now),
        "evening" => hi::evening(),
        _ => hi::morning(now),
    };

    println!("{}", message);
}
