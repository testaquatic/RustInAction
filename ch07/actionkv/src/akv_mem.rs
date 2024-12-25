use clap::{Arg, Command};
use libactionkv::ActionKV;

#[derive(Debug)]
enum Action {
    Get(String),
    Insert { key: String, value: String },
    Delete(String),
    Update { key: String, value: String },
}

#[derive(Debug)]
struct Args {
    fname: String,
    action: Action,
}

fn get_command_args() -> Args {
    let matches = clap::Command::new("akv_eme")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .required(true)
                .help("The file to use as the repository")
                .num_args(1),
        )
        .subcommand(
            Command::new("get")
                .about("Finds the value for a key in the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .num_args(1)
                        .help("The key to get the value for"),
                ),
        )
        .subcommand(
            Command::new("insert")
                .about("Inserts a key-value pair into the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .num_args(1)
                        .help("The key to insert"),
                )
                .arg(
                    Arg::new("value")
                        .required(true)
                        .num_args(1)
                        .help("The value to insert"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes the corresponding key, value pair from the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .num_args(1)
                        .help("The key to delete"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Updates the corresponding key, value pair in the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .num_args(1)
                        .help("The key to update"),
                )
                .arg(
                    Arg::new("value")
                        .required(true)
                        .num_args(1)
                        .help("The new value"),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    let fname = matches.get_one::<String>("file").unwrap().to_string();

    let action = match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            Action::Get(key)
        }
        Some(("insert", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            let value = sub_matches.get_one::<String>("value").unwrap().to_string();
            Action::Insert { key, value }
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            Action::Delete(key)
        }
        Some(("update", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            let value = sub_matches.get_one::<String>("value").unwrap().to_string();
            Action::Update { key, value }
        }
        _ => unreachable!(),
    };

    Args { fname, action }
}

fn main() {
    let args = get_command_args();

    let path = std::path::Path::new(&args.fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match args.action {
        Action::Get(key) => match store.get(key.as_bytes()).expect("failed to get") {
            Some(value) => println!("{:?}", value),
            None => println!("{:?} not found", key),
        },
        Action::Insert { key, value } => {
            store
                .insert(key.as_bytes(), value.as_bytes())
                .expect("failed to insert");
        }
        Action::Delete(key) => store.delete(key.as_bytes()).expect("failed to delete"),
        Action::Update { key, value } => store
            .update(key.as_bytes(), value.as_bytes())
            .expect("failed to update"),
    }
}
