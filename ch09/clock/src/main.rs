use chrono::{DateTime, Utc};
use libclock::{
    clock::Clock,
    command::{get_command_args, Action, Standard},
    ntp::check_time,
};

fn main() {
    let command_args = get_command_args();

    #[cfg(debug_assertions)]
    dbg!(&command_args);

    match &command_args.action {
        Action::Set => {
            let t_ = command_args.datetime.as_ref().unwrap();

            let parser = match command_args.get_standard() {
                Standard::Rfc2822 => DateTime::parse_from_rfc2822,
                Standard::Rfc3339 => DateTime::parse_from_rfc3339,
                _ => unimplemented!(),
            };

            let err_msg = format!(
                "Unable to parse {} according to {:?}",
                t_,
                command_args.get_standard()
            );
            let t = parser(t_).expect(&err_msg);

            Clock::set(t);
        }
        Action::CheckNtp => {
            let offset = check_time().unwrap() as isize;

            let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
            let adjust_ms = chrono::Duration::milliseconds(adjust_ms_ as i64);

            let now = Utc::now() + adjust_ms;

            Clock::set(now);
        }
        Action::Get => (),
    }

    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = &maybe_error.raw_os_error();

    match os_error_code {
        Some(0) => (),
        Some(_) => {
            eprintln!("Unable to set the time: {:?}", maybe_error);
            std::process::exit(1);
        }
        None => (),
    }

    let now = Clock::get();

    match command_args.get_standard() {
        Standard::Rfc2822 => println!("{}", now.to_rfc2822()),
        Standard::Rfc3339 => println!("{}", now.to_rfc3339()),
        Standard::TimeStamp => println!("{}", now.timestamp()),
    }
}
