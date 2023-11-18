cargo_component_bindings::generate!();

use crate::bindings::exports::golem::template::build;
use crate::bindings::exports::golem::template::material;
use crate::bindings::exports::golem::template::routing;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Material {
    id: Box<str>,
    name: String,
}

#[derive(Debug)]
pub struct Part {
    // id: Box<str>,
    material: Arc<Material>,
    amount: u32,
}

#[derive(Debug)]
pub struct Routing {
    id: Box<str>,
    material: Arc<Material>,
    parts: Box<[Part]>,
}

static MATERIALS: Lazy<Mutex<HashMap<String, Material>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static ROUTINGS: Lazy<Mutex<HashMap<String, Routing>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl material::Guest for Component {
    fn get(id: String) -> Option<material::Material> {
        MATERIALS.lock().unwrap().get(&id).cloned()
        // map(|material| material::Material {
        //     name: material.name.to_string()
        // })
    }

    fn add(material: material::Material) -> Result<String, material::Error> {
        let mut materials = MATERIALS.lock().unwrap();

        // todo: generate unique id from material contents
        let id = "123".to_string();

        if materials.contains_key(&id) {
            return Err(material::Error::Duplicate);
        }

        // let material = Material { name: material.name.into_boxed_str() };

        materials.insert(id.clone(), material);

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod material_tests {
    use super::*;
    use crate::bindings::exports::golem::template::material::Guest;

    impl PartialEq for material::Material {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    #[test]
    fn get() {
        clear();

        let id = "123".to_string();

        let material = material::Material {
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

        let material = material::Material {
            name: "foo".to_string(),
        };

        let output = Component::add(material.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_duplicate() {
        clear();

        let material = material::Material {
            name: "foo".to_string(),
        };

        let _ = Component::add(material.clone());
        let output = Component::add(material.clone());

        assert_eq!(output, Err(material::Error::Duplicate));
    }

    fn clear() {
        let mut materials = MATERIALS.lock().unwrap();
        materials.clear();
        drop(materials);
    }
}

impl routing::Guest for Component {
    fn get(id: String) -> Option<routing::Routing> {
        ROUTINGS.lock().unwrap().get(&id).map(|routing| {
            let material = routing.material;

            MATERIALS.lock().unwrap().val
        })
    }

    fn add(routing: routing::Routing) -> Result<String, routing::Error> {
        // todo: verify material ids are valid

        let mut routings = ROUTINGS.lock().unwrap();

        // todo: generate unique id from routing contents
        let id = "123".to_string();

        if routings.contains_key(&id) {
            return Err(routing::Error::Duplicate);
        }

        routings.insert(id.clone(), routing);

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod routing_tests {
    use super::*;
    use crate::bindings::exports::golem::template::routing::Guest;

    impl PartialEq for routing::Part {
        fn eq(&self, other: &Self) -> bool {
            self.material_id == other.material_id && self.amount == other.amount
        }
    }

    impl PartialEq for routing::Routing {
        fn eq(&self, other: &Self) -> bool {
            self.material_id == other.material_id && self.parts == other.parts
        }
    }

    #[test]
    fn get() {
        clear();

        let id = "123".to_string();

        let routing = routing::Routing {
            material_id: id.clone(),
            parts: vec![
                routing::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                routing::Part {
                    material_id: "789".to_string(),
                    amount: 42,
                },
            ],
        };

        let _ = Component::add(routing.clone());
        let output = Component::get(id.clone());

        assert_eq!(output, Some(routing));
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

        let routing = routing::Routing {
            material_id: id.clone(),
            parts: vec![
                routing::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                routing::Part {
                    material_id: "789".to_string(),
                    amount: 42,
                },
            ],
        };

        let output = Component::add(routing.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_duplicate() {
        clear();

        let routing = routing::Routing {
            material_id: "123".to_string(),
            parts: vec![
                routing::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                routing::Part {
                    material_id: "789".to_string(),
                    amount: 42,
                },
            ],
        };

        let _ = Component::add(routing.clone());
        let output = Component::add(routing.clone());

        assert_eq!(output, Err(routing::Error::Duplicate));
    }

    fn clear() {
        let mut routings = ROUTINGS.lock().unwrap();
        routings.clear();
        drop(routings);
    }
}

impl build::Guest for Component {
    fn create(_routing_id: String, _amount: u32) -> Result<(), build::Error> {
        // get routing
        // let _routing = ...::routing::get(routing_id);

        // flatten tree to linear sequence

        // run sequence

        Ok(())
    }
}

#[cfg(test)]
mod build_tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
