use std::str::FromStr;

use clap::Arg;

#[derive(Debug)]
pub struct CommandArgs {
    pub action: Action,
    pub std: Standard,
    pub datetime: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Get,
    Set,
    CheckNtp,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Action::Get),
            "set" => Ok(Action::Set),
            "check-ntp" => Ok(Action::CheckNtp),
            _ => Err(format!("Invalid action: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Standard {
    TimeStamp,
    Rfc2822,
    Rfc3339,
}

impl FromStr for Standard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "timestamp" => Ok(Standard::TimeStamp),
            "rfc2822" => Ok(Standard::Rfc2822),
            "rfc3339" => Ok(Standard::Rfc3339),
            _ => Err(format!("Invalid standard: {}", s)),
        }
    }
}

pub fn get_command_args() -> CommandArgs {
    let matches = clap::Command::new("clock")
        .version("0.1.2")
        .about("Gets and (aspirationally) sets the time.")
        .after_help(
            "Note: UNIX timestamps are parsed as whole seconds since 1st January 1970 0:00:00 UTC. \
            For more accuracy, use another format.",
        )
        .arg(
            Arg::new("action")
                .value_parser(clap::value_parser!(Action))
                .help("The action to perform.")
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("use-standard")
                .value_parser(clap::value_parser!(Standard))
                .help("The standard to use for <datetime>")
                .default_value("rfc3339"),
        )
        .arg(
            Arg::new("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."),
        )
        .get_matches();

    CommandArgs {
        action: matches.get_one::<Action>("action").cloned().unwrap(),
        std: matches.get_one::<Standard>("std").cloned().unwrap(),
        datetime: matches.get_one::<String>("datetime").cloned(),
    }
}

impl CommandArgs {
    pub fn get_action(&self) -> &Action {
        &self.action
    }

    pub fn get_standard(&self) -> &Standard {
        &self.std
    }
}
