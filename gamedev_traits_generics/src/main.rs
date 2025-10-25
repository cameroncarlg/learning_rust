use classes::{Class, Describe, Monster};

// Here we can pass in a trait as an argument
// The trait being game_unit of trait Describe
fn wilderness_interaction(game_unit: &impl Describe) {
    println!(
        "Careful, you've encountered something! {}",
        game_unit.describe_unit()
    )
}

fn main() {
    let berserker = Class {
        name: String::from("Ivanov"),
        role: String::from("Melee DPS"),
        mana_user: false,
        max_hp: 10,
        abilities: vec![
            String::from("Whirlwind"),
            String::from("Bladestorm"),
            String::from("Enrage"),
            String::from("Intervene"),
        ],
    };

    let mage = Class {
        name: String::from("Priests"),
        role: String::from("Ranged DPS"),
        mana_user: true,
        max_hp: 7,
        abilities: vec![
            String::from("Fireball"),
            String::from("Blizzard"),
            String::from("Invoke"),
            String::from("Sleep"),
        ],
    };

    let ural_mountain_breaker = Monster {
        type_of_monster: String::from("bear"),
        max_attack: 6,
        max_hp: 20,
        skin_color: String::from("Black"),
    };

    //println!("{:?}", berserker.describe_unit());
    //println!("{:?}", mage.describe_unit());
    //println!("{:?}", ural_mountain_breaker.describe_unit());
    wilderness_interaction(&berserker);
    wilderness_interaction(&mage);
    wilderness_interaction(&ural_mountain_breaker);
}
