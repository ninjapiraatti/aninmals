#[derive(Debug)]
struct Aninmal {
    name: &'static str,
    age: i32,
}

const ANINMALS: [Aninmal; 21] = [
    Aninmal { name: "Disco turkey", age: 1 },
    Aninmal { name: "Birb", age: 1 },
    Aninmal { name: "Formal chikcen", age: 1 },
    Aninmal { name: "Treefloof", age: 1 },
    Aninmal { name: "Nope rope", age: 1 },
    Aninmal { name: "Danger noodle", age: 1 },
    Aninmal { name: "Sea flap flap", age: 1 },
    Aninmal { name: "Murder log", age: 1 },
    Aninmal { name: "Wizard cow", age: 1 },
    Aninmal { name: "Flopwop", age: 1 },
    Aninmal { name: "Danger zebra", age: 1 },
    Aninmal { name: "Stab rabbit", age: 1 },
    Aninmal { name: "Fart squirrel", age: 1 },
    Aninmal { name: "Blub blub doggo", age: 1 },
    Aninmal { name: "Trouble bubble", age: 1 },
    Aninmal { name: "Aquatic sock puppet", age: 1 },
    Aninmal { name: "Water pistachio", age: 1 },
    Aninmal { name: "Cheese boi", age: 1 },
    Aninmal { name: "Noodle bear", age: 1 },
    Aninmal { name: "Booblesnoot", age: 1 },
    Aninmal { name: "Murder torpedo", age: 1 },
];

pub fn get_random_aninmal_name() {
    let mut all_aninmals: Vec<Aninmal> = Vec::new();
    println!("{:?}", ANINMALS);
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_aninmals() {
        use crate::get_random_aninmal_name;
        use crate::ANINMALS;
        //println!("{:?}", ANINMALS);
        assert_eq!(ANINMALS[0].name, "Disco turkey");
    }
}