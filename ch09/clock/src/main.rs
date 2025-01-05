use chrono::DateTime;
use libclock::{
    clock::Clock,
    command::{get_command_args, Action, Standard},
};

fn main() {
    let command_args = get_command_args();

    #[cfg(debug_assertions)]
    dbg!(&command_args);

    if command_args.action == Action::Set {
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

    let now = Clock::get();
    match command_args.std {
        Standard::TimeStamp => println!("{}", now.timestamp()),
        Standard::Rfc2822 => println!("{}", now.to_rfc2822()),
        Standard::Rfc3339 => println!("{}", now.to_rfc3339()),
    }
}
