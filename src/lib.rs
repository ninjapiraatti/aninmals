use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Aninmal {
    name: &'static str,
    sound: &'static str,
    toesywoesies: i32,
}

const ANINMALS: [Aninmal; 21] = [
    Aninmal { name: "Disco turkey", sound: "Cluck!", toesywoesies: 8 },
    Aninmal { name: "Birb", sound: "Tweet!", toesywoesies: 8 },
    Aninmal { name: "Formal chikcen", sound: "Päkäkäkää!", toesywoesies: 8 },
    Aninmal { name: "Treefloof", sound: "Blerb!", toesywoesies: 20 },
    Aninmal { name: "Nope rope", sound: "Hisssss...", toesywoesies: 0 },
    Aninmal { name: "Danger noodle", sound: "Hisssss...", toesywoesies: 0 },
    Aninmal { name: "Sea flap flap", sound: "Blub", toesywoesies: 0 },
    Aninmal { name: "Murder log", sound: "Snap!", toesywoesies: 20 },
    Aninmal { name: "Wizard cow", sound: "Bah!", toesywoesies: 8 },
    Aninmal { name: "Flopwop", sound: "Squeek!", toesywoesies: 20 },
    Aninmal { name: "Danger zebra", sound: "Roar!", toesywoesies: 20 },
    Aninmal { name: "Stab rabbit", sound: "Squaw!", toesywoesies: 20 },
    Aninmal { name: "Fart squirrel", sound: "Skrit skrit!", toesywoesies: 20 },
    Aninmal { name: "Blub blub doggo", sound: "Blub blub", toesywoesies: 0 },
    Aninmal { name: "Trouble bubble", sound: "...", toesywoesies: 0 },
    Aninmal { name: "Aquatic sock puppet", sound: "Blooorb!", toesywoesies: 0 },
    Aninmal { name: "Water pistachio", sound: "Clack!", toesywoesies: 0 },
    Aninmal { name: "Cheese boi", sound: "Qwik!", toesywoesies: 20 },
    Aninmal { name: "Noodle bear", sound: "Krit krit.", toesywoesies: 20 },
    Aninmal { name: "Booblesnoot", sound: "Om nom.", toesywoesies: 20 },
    Aninmal { name: "Murder torpedo", sound: "Redrum!", toesywoesies: 0 },
];

pub trait Noisemaker {
    fn make_noise(&self);
}

impl Noisemaker for Aninmal {
    fn make_noise(&self) {
        println!("{:?}", self.sound);
    }
}
pub fn get_all_aninmals() -> Vec<Aninmal> {
    get_random_aninmal().make_noise();
    ANINMALS.to_vec()
}

pub fn get_random_aninmal() -> Aninmal {
    let mut rng = thread_rng();
    let x: usize = rng.gen_range(0, ANINMALS.len());
    ANINMALS[x].clone()
}

#[cfg(test)]
mod tests {

    #[test]
    fn has_aninmals() {
        use crate::get_all_aninmals;
        use crate::ANINMALS;
        get_all_aninmals();
        assert_eq!(ANINMALS[0].name, "Disco turkey");
    }
    #[test]
    fn can_has_random_aninmal() {
        use crate::get_random_aninmal;
        let random_aninmal = get_random_aninmal();
        assert_ne!(random_aninmal.name.len(), 0);
        assert_ne!(random_aninmal.sound.len(), 0);
    }
}