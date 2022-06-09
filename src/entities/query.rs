use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
};

use eyre::Result;

use crate::custom_errors::CustomErrors;

use super::Entities;

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entities,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self { entities, map: 0 }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bit_mask(&type_id) {
            self.map |= bit_mask;
            Ok(self)
        } else {
            return Err(CustomErrors::ComponentNotRegistered.into());
        }
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;
        assert_eq!(query.map, 3);
        Ok(())
    }
}
