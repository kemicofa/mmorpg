use super::health::Health;
use super::entity::Entity;

pub struct Creature {
    health: Health,
}

impl Creature {
    fn new() -> Creature {
        Creature {
            health: Health::new()
        }
    }

    fn is_alive(&self) -> bool {
        self.health.is_alive()
    }
}

impl Entity for Creature {
    fn id(&self) -> &str {
        "creature"
    }
    fn update(&self) {
        println!("updating creature")
    }
}

#[cfg(test)]
mod tests {
    use super::Creature;

    #[test]
    fn it_should_initiate() {
        let creature = Creature::new();
        assert_eq!(creature.is_alive(), true);
    }
}