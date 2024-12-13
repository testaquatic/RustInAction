fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yello-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    records
        .skip(1)
        .filter(|record| !record.trim().is_empty())
        .map(|record| {
            let fields = record
                .split(',')
                .map(|field| field.trim())
                .collect::<Vec<_>>();
            if cfg!(debug_assertions) {
                eprintln!("debug: {:?} -> {:?}", record, fields);
            }
            fields
        })
        .for_each(|fields| {
            let name = fields[0];
            if let Ok(length) = fields[1].parse::<f32>() {
                println!("{}, {}cm", name, length);
            }
        });
}                       
