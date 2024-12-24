use clap::{Arg, Command};

#[derive(Debug)]
enum Commands {
    Get(String),
    Insert { key: String, value: String },
    Delete(String),
    Update { key: String, value: String },
}

#[derive(Debug)]
struct Args {
    fname: String,
    command: Commands,
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

    let command = match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            Commands::Get(key)
        }
        Some(("insert", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            let value = sub_matches.get_one::<String>("value").unwrap().to_string();
            Commands::Insert { key, value }
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            Commands::Delete(key)
        }
        Some(("update", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap().to_string();
            let value = sub_matches.get_one::<String>("value").unwrap().to_string();
            Commands::Update { key, value }
        }
        _ => unreachable!(),
    };

    Args { fname, command }
}

fn main() {
    let args = get_command_args();

    let path = std::path::Path::new(&args.fname);

    match args.command {
        Commands::Get(key) => {
            let value = store.get(&key).unwrap();
            println!("{}", value);
        }
        Commands::Insert { key, value } => {
            store.insert(key, value);
        }
        Commands::Delete(key) => {
            store.remove(&key);
        }
        Commands::Update { key, value } => {
            store.update(&key, value);
        }
    }
}
