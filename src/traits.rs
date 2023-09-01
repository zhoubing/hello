use rand::{seq::SliceRandom, Rng};


struct Dwarf {

}
struct Elf {

}

struct Human {

}

impl Dwarf {
    pub fn func1() {
        println!("dwarf func1");
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        println!("dwarf compete");
        return 0.1
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        println!("elf compete");
        return 0.3
    }  
}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        println!("human compete");
        return 0.5
    }
}

trait Enchanter {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability = self.competency();
        let is_success = rand::thread_rng().gen_bool(probability);
        if is_success {
            println!("{:?} enchant success", thing);
        } else {
            println!("{:?} enchant falied", thing);
        }
    }
}

pub fn run() {
    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};
    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    party.get(0).unwrap().competency();

    let sc = party.choose(&mut rand::thread_rng()).unwrap();
    sc.enchant(&mut Thing::Sword);
}