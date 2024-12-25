use std::collections::HashMap;

use clap::{Arg, Command};
use libactionkv::{ActionKV, ByteStr, ByteString};

#[derive(Debug)]
enum Action {
    Get(String),
    Delete(String),
    Insert { key: String, value: String },
    Update { key: String, value: String },
}

#[derive(Debug)]
struct Args {
    fname: String,
    action: Action,
}

fn get_args() -> Args {
    let matches = Command::new("akv_disk")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .required(true)
                .help("The file to use")
                .num_args(1),
        )
        .subcommand(
            Command::new("get")
                .about("Get a value for a key from the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .help("The key to get")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a key-value pair from the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .help("The key to delete")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("insert")
                .about("Insert a kev-value pair into the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .help("The key to insert")
                        .num_args(1),
                )
                .arg(
                    Arg::new("value")
                        .required(true)
                        .help("The value to insert")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Updates the corresponding key-value pair in the repository")
                .arg(
                    Arg::new("key")
                        .required(true)
                        .help("The key to update")
                        .num_args(1),
                )
                .arg(
                    Arg::new("value")
                        .required(true)
                        .help("The value to update")
                        .num_args(1),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    let fname = matches.get_one::<String>("file").unwrap().clone();
    match matches.subcommand() {
        Some(("get", sub_matches)) => Args {
            fname,
            action: Action::Get(sub_matches.get_one::<String>("key").unwrap().to_string()),
        },
        Some(("delete", sub_matches)) => Args {
            fname,
            action: Action::Delete(sub_matches.get_one::<String>("key").unwrap().to_string()),
        },
        Some(("insert", sub_matches)) => Args {
            fname,
            action: Action::Insert {
                key: sub_matches.get_one::<String>("key").unwrap().to_string(),
                value: sub_matches.get_one::<String>("value").unwrap().to_string(),
            },
        },
        Some(("update", sub_matches)) => Args {
            fname,
            action: Action::Update {
                key: sub_matches.get_one::<String>("key").unwrap().to_string(),
                value: sub_matches.get_one::<String>("value").unwrap().to_string(),
            },
        },
        _ => unreachable!(),
    }
}

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";
    let args = get_args();

    let path = std::path::Path::new(&args.fname);
    let mut a = ActionKV::open(path).expect("unable to open file");

    a.load().expect("unable to load data");

    match &args.action {
        Action::Get(key) => {
            let index_as_bytes = a.get(&INDEX_KEY).unwrap().unwrap();
            let index_decoded = bincode::deserialize::<HashMap<ByteString, u64>>(&index_as_bytes);

            let index = index_decoded.unwrap();

            match index.get(key.as_bytes()) {
                Some(&i) => {
                    let kv = a.get_at(i).unwrap();
                    println!("{:?}", kv.value);
                }
                None => eprintln!("{key:?} not found"),
            }
        }
        Action::Delete(key) => {
            a.delete(key.as_bytes()).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }
        Action::Insert { key, value } => {
            a.insert(key.as_bytes(), value.as_bytes()).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }
        Action::Update { key, value } => {
            a.update(key.as_bytes(), value.as_bytes()).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }
    }

    #[cfg(debug_assertions)]
    println!("{args:?}");
}
