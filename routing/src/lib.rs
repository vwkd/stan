cargo_component_bindings::generate!();
use bindings::exports::golem::routing::api;

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static ROUTINGS: Lazy<Mutex<HashMap<String, api::Routing>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl api::Guest for Component {
    fn get(id: String) -> Option<api::Routing> {
        ROUTINGS.lock().unwrap().get(&id).cloned()
    }

    fn add(routing: api::Routing) -> Result<String, api::Error> {
        // todo: verify material ids are valid

        let mut routings = ROUTINGS.lock().unwrap();

        // todo: generate unique id from routing contents
        let id = "123".to_string();

        if routings.contains_key(&id) {
            return Err(api::Error::Duplicate);
        }

        routings.insert(id.clone(), routing);

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::bindings::exports::golem::routing::api::Guest;
    use super::*;

    impl PartialEq for api::Part {
        fn eq(&self, other: &Self) -> bool {
            self.material_id == other.material_id && self.amount == other.amount
        }
    }

    impl PartialEq for api::Routing {
        fn eq(&self, other: &Self) -> bool {
            self.material_id == other.material_id && self.parts == other.parts
        }
    }

    #[test]
    fn get() {
        clear();

        let id = "123".to_string();

        let routing = api::Routing {
            material_id: id.clone(),
            parts: vec![
                api::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                api::Part {
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

        let routing = api::Routing {
            material_id: id.clone(),
            parts: vec![
                api::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                api::Part {
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

        let routing = api::Routing {
            material_id: "123".to_string(),
            parts: vec![
                api::Part {
                    material_id: "456".to_string(),
                    amount: 21,
                },
                api::Part {
                    material_id: "789".to_string(),
                    amount: 42,
                },
            ],
        };

        let _ = Component::add(routing.clone());
        let output = Component::add(routing.clone());

        assert_eq!(output, Err(api::Error::Duplicate));
    }

    fn clear() {
        let mut routings = ROUTINGS.lock().unwrap();
        routings.clear();
        drop(routings);
    }
}
