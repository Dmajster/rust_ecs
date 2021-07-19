use std::{any::TypeId, fmt::Debug};

use itertools::multizip;

use crate::component_group::{
    Component, ComponentGroup, IntoComponentGroup, IntoComponentGroupKey,
};

#[derive(Debug, Default)]
pub struct ComponentGroupStore {
    component_groups: Vec<ComponentGroup>,
}

impl ComponentGroupStore {
    pub fn push<I>(&mut self, values: I)
    where
        I: IntoComponentGroupPush,
    {
        values.push(self);
    }

    pub fn get_1<A>(&self) -> impl Iterator<Item = &A>
    where
        A: Component,
    {
        let searched_key = <(A,)>::get_key();

        let containing_component_groups = self
            .component_groups
            .iter()
            .filter(move |cg| searched_key.is_subset(cg.get_key()));

        containing_component_groups
            .map(|cg| {
                cg.get_store()
                    .get(&TypeId::of::<A>())
                    .unwrap()
                    .downcast_ref::<Vec<A>>()
                    .unwrap()
                    .iter()
            })
            .flatten()
    }

    pub fn get_2<A, B>(&self) -> impl Iterator<Item = (&A, &B)>
    where
        A: Component,
        B: Component,
    {
        let searched_key = <(A, B)>::get_key();

        let containing_component_groups = self
            .component_groups
            .iter()
            .filter(move |cg| searched_key.is_subset(cg.get_key()));

        containing_component_groups
            .map(|cg| {
                let iter_a = cg
                    .get_store()
                    .get(&TypeId::of::<A>())
                    .unwrap()
                    .downcast_ref::<Vec<A>>()
                    .unwrap()
                    .iter();

                let iter_b = cg
                    .get_store()
                    .get(&TypeId::of::<B>())
                    .unwrap()
                    .downcast_ref::<Vec<B>>()
                    .unwrap()
                    .iter();

                iter_a.zip(iter_b)
            })
            .flatten()
    }

    pub fn get_3<A, B, C>(&self) -> impl Iterator<Item = (&A, &B, &C)>
    where
        A: Component,
        B: Component,
        C: Component,
    {
        let searched_key = <(A, B, C)>::get_key();

        let containing_component_groups = self
            .component_groups
            .iter()
            .filter(move |cg| searched_key.is_subset(cg.get_key()));

        containing_component_groups
            .map(|cg| {
                let iter_a = cg
                    .get_store()
                    .get(&TypeId::of::<A>())
                    .unwrap()
                    .downcast_ref::<Vec<A>>()
                    .unwrap()
                    .iter();

                let iter_b = cg
                    .get_store()
                    .get(&TypeId::of::<B>())
                    .unwrap()
                    .downcast_ref::<Vec<B>>()
                    .unwrap()
                    .iter();

                let iter_c = cg
                    .get_store()
                    .get(&TypeId::of::<C>())
                    .unwrap()
                    .downcast_ref::<Vec<C>>()
                    .unwrap()
                    .iter();

                multizip((iter_a, iter_b, iter_c))
            })
            .flatten()
    }
}

pub trait IntoComponentGroupPush {
    fn push(self, store: &mut ComponentGroupStore);
}

impl<A> IntoComponentGroupPush for (A,)
where
    A: Component,
{
    fn push(self, store: &mut ComponentGroupStore) {
        let component_group_search_result =
            store
                .component_groups
                .iter_mut()
                .find(|compared_component_group| {
                    compared_component_group.get_key() == &Self::get_key()
                });

        if let Some(component_group) = component_group_search_result {
            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<A>())
                .unwrap()
                .downcast_mut::<Vec<A>>()
                .unwrap()
                .push(self.0);
        } else {
            store.component_groups.push(self.into_component_group());
        }
    }
}

impl<A, B> IntoComponentGroupPush for (A, B)
where
    A: Component,
    B: Component,
{
    fn push(self, store: &mut ComponentGroupStore) {
        let component_group_search_result =
            store
                .component_groups
                .iter_mut()
                .find(|compared_component_group| {
                    compared_component_group.get_key() == &Self::get_key()
                });

        if let Some(component_group) = component_group_search_result {
            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<A>())
                .unwrap()
                .downcast_mut::<Vec<A>>()
                .unwrap()
                .push(self.0);

            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<B>())
                .unwrap()
                .downcast_mut::<Vec<B>>()
                .unwrap()
                .push(self.1);
        } else {
            store.component_groups.push(self.into_component_group());
        }
    }
}

impl<A, B, C> IntoComponentGroupPush for (A, B, C)
where
    A: Component,
    B: Component,
    C: Component,
{
    fn push(self, store: &mut ComponentGroupStore) {
        let component_group_search_result =
            store
                .component_groups
                .iter_mut()
                .find(|compared_component_group| {
                    compared_component_group.get_key() == &Self::get_key()
                });

        if let Some(component_group) = component_group_search_result {
            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<A>())
                .unwrap()
                .downcast_mut::<Vec<A>>()
                .unwrap()
                .push(self.0);

            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<B>())
                .unwrap()
                .downcast_mut::<Vec<B>>()
                .unwrap()
                .push(self.1);

            component_group
                .get_store_mut()
                .get_mut(&TypeId::of::<C>())
                .unwrap()
                .downcast_mut::<Vec<C>>()
                .unwrap()
                .push(self.2);
        } else {
            store.component_groups.push(self.into_component_group());
        }
    }
}
