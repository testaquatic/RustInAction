use rand::Rng;

#[derive(Debug)]
pub struct Dwarf;

#[derive(Debug)]
pub struct Elf;

#[derive(Debug)]
pub struct Human;

#[derive(Debug)]
pub enum Thing {
    Sword,
    Trinket,
}

pub trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);

        print!("{self:?} mutters incoherently. ");
        if spell_is_successful {
            println!("The {thing:?} glows brightly.");
        } else {
            println!("The {thing:?} fizzes, then turns into a worthless trinket.");
            *thing = Thing::Trinket;
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}
