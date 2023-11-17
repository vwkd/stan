cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static MATERIALS: Lazy<Mutex<HashMap<String, api::Material>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl api::Guest for Component {
    fn get(id: String) -> Option<api::Material> {
        MATERIALS.lock().unwrap().get(&id).cloned()
    }

    fn add(material: api::Material) -> Result<String, api::Error> {
        let mut materials = MATERIALS.lock().unwrap();

        // todo: generate unique id from material contents
        let id = "123".to_string();

        if materials.contains_key(&id) {
            return Err(api::Error::Duplicate);
        }

        materials.insert(id.clone(), material);

        Ok(id)
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

        let id = "123".to_string();

        let material = api::Material {
            name: "foo".to_string(),
        };

        let _ = Component::add(material.clone());
        let output = Component::get(id.clone());

        assert_eq!(output, Some(material));
    }

    #[test]
    fn get_invalid() {
        clear();

        let output = Component::get("123".to_string());

        assert_eq!(output, None);
    }

    #[test]
    fn add() {
        clear();

        let id = "123".to_string();

        let material = api::Material {
            name: "foo".to_string(),
        };

        let output = Component::add(material.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_duplicate() {
        clear();

        let material = api::Material {
            name: "foo".to_string(),
        };

        let _ = Component::add(material.clone());
        let output = Component::add(material.clone());

        assert_eq!(output, Err(api::Error::Duplicate));
    }

    fn clear() {
        let mut materials = MATERIALS.lock().unwrap();
        materials.clear();
        drop(materials);
    }
}
