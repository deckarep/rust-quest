//http://blog.jwilm.io/from-str-to-cow/

extern crate rand;
extern crate ansi_term;

use std::fmt;

use rand::Rng;
use ansi_term::Colour;

struct Item {
    name: String,
    hp: i32,
}

struct Weapon {
    name: String,
    damage: i32,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Monster {
    name: String,
    hp: i32,
}

impl Monster {
    pub fn new(name: &str) -> Monster {
        let m = Monster {
            name: name.into(),
            hp: rand::thread_rng().gen_range(5, 100),
        };

        println!("{} spawns with a hp of {}", name, m.hp);
        m
    }

    fn attack(&self, hero: &mut Hero) -> bool {
        let (isDead, dmg) = hero.damage();
        if isDead {
            println!("{} {} causes a {} point fatal blow to {} with his scorched teeth.",
                     Colour::Purple.paint("Action:"),
                     Colour::Green.paint(self.name.clone()),
                     dmg,
                     hero.name);
            return true;
        }

        println!("{} {} lunges toward {}, shoots an acidic ooze and causes {} points of damage.",
                 Colour::Purple.paint("Action:"),
                 Colour::Green.paint(self.name.clone()),
                 hero.name,
                 dmg);

        false
    }

    fn damage(&mut self) -> (bool, i32) {
        let rn = rand::thread_rng().gen_range(0, 3);
        self.hp -= rn;

        if self.hp <= 0 {
            return (true, rn);
        }

        (false, rn)
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.hp)
    }
}

struct Hero {
    name: String,
    active_weapon: Weapon,
    inventory: Vec<Item>,
    hp: i32,
}

impl Hero {
    pub fn new(name: &str) -> Hero {
        Hero {
            name: name.into(),
            hp: 50,
            inventory: Vec::new(),
            active_weapon: Weapon {
                name: "Sword of Doom".into(),
                damage: 13,
            },
        }
    }

    fn damage(&mut self) -> (bool, i32) {
        let rn = rand::thread_rng().gen_range(0, 3);
        self.hp -= rn;

        if self.hp <= 0 {
            return (true, rn);
        }

        (false, rn)
    }

    fn attack(&self, monster: Option<&mut Monster>) -> bool {
        match monster {
            Some(m) => {
                let (isDead, dmg) = m.damage();
                if isDead {
                    println!("{} {} causes a {} point fatal blow to the {} with his {}",
                             Colour::Purple.paint("Action:"),
                             Colour::Green.paint(self.name.clone()),
                             dmg,
                             m.name,
                             self.active_weapon);
                    return true;
                }

                println!("{} {} jumps toward the {}, swings his {} in fury and causes {} points of damage.",
                         Colour::Purple.paint("Action:"),
                         Colour::Green.paint(self.name.clone()),
                         m.name,
                         self.active_weapon,
                         dmg);
            }
            None => {
                println!("{} {} flails his {} around like a fool.",
                         Colour::Purple.paint("Action:"),
                         Colour::Green.paint(self.name.clone()),
                         self.active_weapon)
            }
        }
        false
    }
}

fn main() {
    let mut he_man = Hero::new("He Man");
    let mut monster = Monster::new("Hash Slinging Slasher");

    loop {
        let battle_over = he_man.attack(Some(&mut monster));
        if battle_over {
            break;
        }

        let battle_over = monster.attack(&mut he_man);
        if battle_over {
            break;
        }
    }
}
