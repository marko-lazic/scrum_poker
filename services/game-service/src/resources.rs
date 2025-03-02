use std::any::{Any, TypeId};
use std::collections::HashMap;

// Resource trait to mark types that can be used as resources
pub trait Resource: 'static + Send + Sync {}

// Implement Resource for any type that is 'static + Send + Sync
impl<T: 'static + Send + Sync> Resource for T {}

// Simple resource container (similar to Bevy's World for resources)
pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: Resource>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: Resource>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_ref::<T>())
    }

    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_mut::<T>())
    }

    pub fn contains<T: Resource>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    pub fn remove<T: Resource>(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.resources
            .remove(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_ref::<T>().cloned())
    }

    // Similar to Bevy's world.resource_scope
    pub fn resource_scope<T: Resource, R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.get::<T>().map(|resource| f(resource))
    }

    pub fn resource_scope_mut<T: Resource, R>(&mut self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.get_mut::<T>().map(|resource| f(resource))
    }
}
