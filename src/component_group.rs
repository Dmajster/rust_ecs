use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
};

pub trait Component: 'static {}
impl<A> Component for A where A: 'static {}

pub type ComponentGroupKey = HashSet<TypeId>;
pub type ComponentGroupStore = HashMap<TypeId, Box<dyn Any>>;

#[derive(Debug)]
pub struct ComponentGroup {
    key: ComponentGroupKey,
    store: ComponentGroupStore,
}

impl ComponentGroup {
    pub fn new(key: ComponentGroupKey, store: ComponentGroupStore) -> Self {
        Self { key, store }
    }

    pub fn get_key(&self) -> &ComponentGroupKey {
        &self.key
    }

    pub fn get_store(&self) -> &ComponentGroupStore {
        &self.store
    }

    pub fn get_store_mut(&mut self) -> &mut ComponentGroupStore {
        &mut self.store
    }
}

pub trait IntoComponentGroup {
    fn into_component_group(self) -> ComponentGroup;
}

impl<A> IntoComponentGroup for (A,)
where
    A: Component,
{
    fn into_component_group(self) -> ComponentGroup {
        let key = Self::get_key();

        let mut store = ComponentGroupStore::new();
        store.insert(TypeId::of::<A>(), Box::new(vec![self.0]));

        ComponentGroup::new(key, store)
    }
}



impl<A, B> IntoComponentGroup for (A, B)
where
    A: Component,
    B: Component,
{
    fn into_component_group(self) -> ComponentGroup {
        let key = Self::get_key();

        let mut store = ComponentGroupStore::new();
        store.insert(TypeId::of::<A>(), Box::new(vec![self.0]));
        store.insert(TypeId::of::<B>(), Box::new(vec![self.1]));

        ComponentGroup::new(key, store)
    }
}

impl<A, B, C> IntoComponentGroup for (A, B, C)
where
    A: Component,
    B: Component,
    C: Component,
{
    fn into_component_group(self) -> ComponentGroup {
        let key = Self::get_key();

        let mut store = ComponentGroupStore::new();
        store.insert(TypeId::of::<A>(), Box::new(vec![self.0]));
        store.insert(TypeId::of::<B>(), Box::new(vec![self.1]));
        store.insert(TypeId::of::<C>(), Box::new(vec![self.2]));

        ComponentGroup::new(key, store)
    }
}

pub trait IntoComponentGroupKey {
    fn get_key() -> ComponentGroupKey;
}

impl<A> IntoComponentGroupKey for (A,)
where
    A: Component,
{
    fn get_key() -> ComponentGroupKey {
        let mut key = ComponentGroupKey::new();
        key.insert(TypeId::of::<A>());
        key
    }
}

impl<A, B> IntoComponentGroupKey for (A, B)
where
    A: Component,
    B: Component,
{
    fn get_key() -> ComponentGroupKey {
        let mut key = ComponentGroupKey::new();
        key.insert(TypeId::of::<A>());
        key.insert(TypeId::of::<B>());
        key
    }
}

impl<A, B, C> IntoComponentGroupKey for (A, B, C)
where
    A: Component,
    B: Component,
    C: Component,
{
    fn get_key() -> ComponentGroupKey {
        let mut key = ComponentGroupKey::new();
        key.insert(TypeId::of::<A>());
        key.insert(TypeId::of::<B>());
        key.insert(TypeId::of::<C>());
        key
    }
}
