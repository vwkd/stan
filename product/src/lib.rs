cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static PRODUCTS: Lazy<Mutex<HashMap<String, api::Product>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl api::Guest for Component {
    fn get(id: String) -> Option<api::Product> {
        PRODUCTS.lock().unwrap().get(&id).cloned()
    }

    fn add(product: api::Product) -> Result<(), api::Error> {
        let mut products = PRODUCTS.lock().unwrap();

        if products.contains_key(&product.id) {
            return Err(api::Error::Duplicate);
        }

        products.insert(product.id.clone(), product);

        Ok(())
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::*;
    use crate::bindings::exports::golem::template::api::Guest;

    impl PartialEq for api::Product {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.name == other.name
        }
    }

    #[test]
    fn get() {
        clear();

        let product = api::Product {
            id: "123".to_string(),
            name: "foo".to_string(),
        };

        let _ = Component::add(product.clone());
        let output = Component::get(product.id.clone());

        assert_eq!(output, Some(product));
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

        let product = api::Product {
            id: "123".to_string(),
            name: "foo".to_string(),
        };

        let output = Component::add(product.clone());

        assert_eq!(output, Ok(()));
    }

    #[test]
    fn add_duplicate() {
        clear();

        let product = api::Product {
            id: "123".to_string(),
            name: "foo".to_string(),
        };

        let _ = Component::add(product.clone());
        let output = Component::add(product.clone());

        assert_eq!(output, Err(api::Error::Duplicate));
    }

    fn clear() {
        let mut products = PRODUCTS.lock().unwrap();
        products.clear();
        drop(products);
    }
}
