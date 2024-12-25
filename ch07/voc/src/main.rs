use std::collections::BTreeMap;

fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middelburg");
    voc.insert(540_000, "Enkhuizen");
    voc.insert(469_400, "Delft");
    voc.insert(266_868, "Hoorn");
    voc.insert(173_000, "Rotterdam");

    voc.iter()
        .for_each(|(guilders, kamer)| println!("chamber {kamer} invested {guilders}"));

    println!("smaller chambers: ");
    voc.range(0..500_000)
        .for_each(|(_guilders, kamer)| print!("{} ", kamer));
    println!();
}
