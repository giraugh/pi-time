use chrono::{Duration, Local, NaiveTime, Timelike};
use clap::Parser;
use humantime::format_duration;

const PI_TIMES: [(u32, u32, u32); 2] = [(3, 14, 0), (15, 14, 0)];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// #[command(author = "Ewan Breakey (giraugh) <https://ewanb.me>")]
// #[command(about = "Find out when its pi time", long_about = None)]
struct CliArgs {
    #[arg(short = 'e', long)]
    hide_emoji: bool,

    #[arg(short, long)]
    detailed_duration: bool,
}

fn main() {
    // Parse CLI args
    let args = CliArgs::parse();

    // Print out pi time stuff
    if is_pi_time() {
        println!("{}It's pi time!", if !args.hide_emoji { "🥧 " } else { "" });
    } else {
        let time_until = duration_until_pi_time().to_std().unwrap();
        let time_until = if args.detailed_duration {
            time_until
        } else {
            truncate_duration_to_seconds(time_until)
        };
        println!("{} until pi time", format_duration(time_until));
    }
}

/**
Determine whether the current time is pi time
*/
fn is_pi_time() -> bool {
    let current_time = Local::now().time();
    let current_pi_time = PI_TIMES
        .iter()
        .map(|&(h, m, s)| NaiveTime::from_hms(h, m, s))
        .find(|&pi_time| {
            pi_time.hour() == current_time.hour() && pi_time.minute() == current_time.minute()
        });
    current_pi_time.is_some()
}

/**
Determine time duration until next pi time.
has a special case when the next pi time is tomorrow.
*/
fn duration_until_pi_time() -> Duration {
    let current_time = Local::now().time();
    let durations_today = PI_TIMES
        .iter()
        .map(|&(h, m, s)| NaiveTime::from_hms(h, m, s))
        .filter(|&pi_time| pi_time > current_time)
        .map(|pi_time| pi_time - current_time)
        .min();
    if let Some(smallest_duration) = durations_today {
        smallest_duration
    } else {
        let time_until_tomorrow = NaiveTime::from_hms(23, 59, 59) - current_time;
        let time_from_midnight_til_pi_time =
            Duration::hours(PI_TIMES[0].0.into()) + Duration::minutes(PI_TIMES[0].1.into());
        time_until_tomorrow + time_from_midnight_til_pi_time
    }
}

/**
Truncate duration to remove sub-second (e.g millisecond and microsecond) data.
*/
fn truncate_duration_to_seconds(duration: std::time::Duration) -> std::time::Duration {
    std::time::Duration::from_secs(duration.as_secs())
}
