use std::any::Any;

use entities::Entities;
use resource::Resource;

pub mod custom_errors;
mod entities;
mod resource;

#[derive(Default, Debug)]
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

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }
}
