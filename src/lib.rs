use std::any::Any;

use entities::{query::Query, Entities};
use resource::Resource;

mod custom_errors;
mod entities;
mod resource;

#[derive(Default)]
pub struct World {
    resources: Resource,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /**
    Query for a resource and get a mutable reference to it. The type of the resource must be added in so that we can find it.
    ```
    use brooks_builds_ecs::World;
    let mut world = World::new();
    world.add_resource(10_u32);
    {
        let resource = world.get_resource_mut::<u32>().unwrap();
        *resource += 1;
    }
    let resource = world.get_resource::<u32>().unwrap();
    assert_eq!(*resource, 11);
    ```
    */
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /**
     * This will remove the resource from the world, and it doesn't care if the resource exists at this point in time.
     */
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }
}

#[cfg(test)]
mod tests {}
