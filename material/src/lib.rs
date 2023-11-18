cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api;
use std::sync::Mutex;

static MATERIALS: Mutex<Vec<api::Material>> = Mutex::new(Vec::new());

struct Component;

impl api::Guest for Component {
    fn get(id: u64) -> Option<api::Material> {
        let id: usize = id.try_into().expect("64-bit OS");

        let materials = MATERIALS.lock().unwrap();

        materials.get(id).cloned()
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

        materials.push(material);

        Ok(id)
    }

    fn update(id: u64, material: api::MaterialUpdate) -> Result<api::Material, api::ErrorUpdate> {
        let id: usize = id.try_into().expect("64-bit OS");

        let mut materials = MATERIALS.lock().unwrap();

        let mat = materials.get_mut(id).ok_or(api::ErrorUpdate::NotExist)?;

        let material_old = mat.clone();

        if let Some(name) = material.name {
            mat.name = name
        }
        // ... remaining

        Ok(material_old)
    }

    fn delete(id: u64) -> Result<api::Material, api::ErrorDelete> {
        let id: usize = id.try_into().expect("64-bit OS");

        let mut materials = MATERIALS.lock().unwrap();

        if id >= materials.len() {
            return Err(api::ErrorDelete::NotExist);
        }

        Ok(materials.remove(id))
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::*;
    use crate::bindings::exports::golem::template::api::Guest;

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

    // todo
    // #[test]
    // fn add_max_capacity() {
    //     clear();

    //     // ...

    //     assert_eq!(output, Err(api::ErrorAdd::MaxCapacity));
    // }

    #[test]
    fn update() {
        clear();

        let id = 0;
        let name = "foo".to_string();

        let material = api::MaterialAdd { name: name.clone() };

        let _ = Component::add(material.clone());

        let material_new = api::MaterialUpdate {
            name: Some("bar".to_string()),
        };

        let output = Component::update(id, material_new);

        let material = api::Material { id, name };

        assert_eq!(output, Ok(material));
    }

    #[test]
    fn update_not_exist() {
        clear();

        let id = 123;

        let material = api::MaterialUpdate {
            name: Some("foo".to_string()),
        };

        let output = Component::update(id, material);

        assert_eq!(output, Err(api::ErrorUpdate::NotExist));
    }

    #[test]
    fn delete() {
        clear();

        let id = 0;
        let name = "foo".to_string();

        let material = api::MaterialAdd { name: name.clone() };

        let _ = Component::add(material.clone());
        let output = Component::delete(id);

        let material = api::Material { id, name };

        assert_eq!(output, Ok(material));
    }

    #[test]
    fn delete_not_exist() {
        clear();

        let id = 123;

        let output = Component::delete(id);

        assert_eq!(output, Err(api::ErrorDelete::NotExist));
    }

    fn clear() {
        let mut materials = MATERIALS.lock().unwrap();
        materials.clear();
        drop(materials);
    }
}
