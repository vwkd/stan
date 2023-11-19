cargo_component_bindings::generate!();
use crate::bindings::exports::golem::material::api;
use std::sync::Mutex;

static MATERIALS: Mutex<Vec<Option<api::Material>>> = Mutex::new(Vec::new());

struct Component;

impl api::Guest for Component {
    fn get(id: u64) -> Option<api::Material> {
        let id: usize = id.try_into().expect("64-bit OS");

        let materials = MATERIALS.lock().unwrap();

        materials.get(id).cloned().unwrap_or(None)
    }

    fn add(material: api::MaterialAdd) -> Result<u64, api::ErrorAdd> {
        let mut materials = MATERIALS.lock().unwrap();

        let id = materials.len();
        let id: u64 = id.try_into().expect("64-bit OS");

        if id == u64::MAX {
            return Err(api::ErrorAdd::MaxCapacity);
        }

        let material = api::Material {
            id,
            name: material.name,
        };

        materials.push(Some(material));

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::*;
    use crate::bindings::exports::golem::material::api::Guest;

    impl PartialEq for api::Material {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    #[test]
    fn get() {
        clear();

        let id = 0;
        let name = "foo".to_string();

        let material = api::MaterialAdd { name: name.clone() };

        let _ = Component::add(material);
        let output = Component::get(id);

        let material = api::Material { id, name };

        assert_eq!(output, Some(material));
    }

    #[test]
    fn get_not_exist() {
        clear();

        let output = Component::get(123);

        assert_eq!(output, None);
    }

    #[test]
    fn add() {
        clear();

        let id = 0;

        let material = api::MaterialAdd {
            name: "foo".to_string(),
        };

        let output = Component::add(material.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_incrementing_ids() {
        clear();

        let id = 1;

        let material = api::MaterialAdd {
            name: "foo".to_string(),
        };

        let _ = Component::add(material.clone());
        let output = Component::add(material.clone());

        assert_eq!(output, Ok(id));
    }

    // todo
    // #[test]
    // fn add_max_capacity() {
    //     clear();

    //     // ...

    //     assert_eq!(output, Err(api::ErrorAdd::MaxCapacity));
    // }

    fn clear() {
        let mut materials = MATERIALS.lock().unwrap();
        materials.clear();
        drop(materials);
    }
}
