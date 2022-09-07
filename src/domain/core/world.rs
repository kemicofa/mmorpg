use super::entity::Entity;

pub struct World {
    entities: Vec<Box<dyn Entity>>
}

impl World {
    fn number_of_entities(&self) -> usize {
        self.entities.len()
    }
    fn new() -> World {
        World{
            entities: Vec::new()
        }
    }
    fn tick(&self) {
        for entity in self.entities.as_slice() {
            println!("{}", entity.id());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn it_should_initiate() {
        let world = World::new();
        assert_eq!(world.number_of_entities(), 0);
    }
}