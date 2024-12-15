#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

impl From<&str> for Event {
    fn from(value: &str) -> Self {
        match value {
            "UPDATE" | "update" => Event::Update,
            "DELETE" | "delete" => Event::Delete,
            _ => Event::Unknown,
        }
    }
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let mut parts = line.splitn(2, ' ');
    if let Some(event_) = parts.next() {
        if let Some(rest_) = parts.next() {
            return (event_.into(), rest_.into());
        }
    }

    (Event::Unknown, line.into())
}

fn main() {
    let log = r#"BEGIN Transaction XK342
UPDATE 234:LS/32231 {"price": 31.00} -> {"price": 40.00}
DELETE 342:LO/22111"#;

    log.lines()
        .map(parse_log)
        .for_each(|parse_result| println!("{parse_result:?}"));
}
