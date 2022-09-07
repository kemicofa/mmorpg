pub const MAX_HEALTH: u64 = 10000;

pub struct Health {
    points: u64,
}

impl Health {
    pub fn new() -> Health {
        Health { points: MAX_HEALTH }
    }

    pub fn is_alive(&self) -> bool {
        self.points > 0
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_HEALTH, Health};

    #[test]
    fn it_should_initiate() {
        let health = Health::new();
        assert_eq!(health.points, MAX_HEALTH);
    }
}