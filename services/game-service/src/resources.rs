use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

// Resource trait to mark types that can be used as resources
pub trait Resource: 'static + Send + Sync {}

// Implement Resource for any type that is 'static + Send + Sync
impl<T: 'static + Send + Sync> Resource for T {}

// Simple resource container (similar to Bevy's World for resources)
pub struct Resources {
    pub resources: HashMap<TypeId, RefCell<Box<dyn Any + Send + Sync>>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: Resource>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(resource)));
    }

    pub fn get<T: Resource>(&self) -> Option<Ref<T>> {
        self.resources
            .get(&TypeId::of::<T>())
            .map(|cell| Ref::map(cell.borrow(), |b| b.downcast_ref::<T>().unwrap()))
    }

    pub fn get_mut<T: Resource>(&self) -> Option<RefMut<T>> {
        self.resources
            .get(&TypeId::of::<T>())
            .map(|cell| RefMut::map(cell.borrow_mut(), |b| b.downcast_mut::<T>().unwrap()))
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
            .map(|cell| cell.into_inner().downcast_ref::<T>().unwrap().clone())
    }

    // Similar to Bevy's world.resource_scope
    pub fn resource_scope<T: Resource, R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.get::<T>().map(|resource| f(&*resource))
    }

    pub fn resource_scope_mut<T: Resource, R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.get_mut::<T>().map(|mut resource| f(&mut *resource))
    }
}
