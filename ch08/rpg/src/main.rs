use std::vec;

use librpg::{Dwarf, Elf, Enchanter, Human, Thing};
use rand::seq::SliceRandom;

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf;
    let e = Elf;
    let h = Human;

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}
