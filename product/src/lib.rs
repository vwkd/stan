cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static PRODUCTS: Lazy<Mutex<HashMap<String, Product>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl Guest for Component {
    fn get(id: String) -> Option<Product> {
        PRODUCTS.lock().unwrap().get(&id).cloned()
    }

    fn add(product: Product) -> Result<(), Error> {
        let mut products = PRODUCTS.lock().unwrap();

        if products.contains_key(&product.id) {
            return Err(Error::Duplicate);
        }

        products.insert(product.id.clone(), product);

        Ok(())
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::*;

    #[test]
    fn get() {}
}
