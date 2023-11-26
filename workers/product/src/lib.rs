cargo_component_bindings::generate!();
use bindings::exports::stan::product::api;
use std::sync::Mutex;

static PRODUCTS: Mutex<Vec<Option<api::Product>>> = Mutex::new(Vec::new());

struct Component;

impl api::Guest for Component {
    fn get(id: api::Id) -> Option<api::Product> {
        let id: usize = id.try_into().expect("64-bit OS");

        let products = PRODUCTS.lock().unwrap();

        products.get(id).cloned().unwrap_or(None)
    }

    fn add(product: api::ProductAdd) -> Result<api::Id, api::ErrorAdd> {
        let mut products = PRODUCTS.lock().unwrap();

        let id = products.len();
        let id: api::Id = id.try_into().expect("64-bit OS");

        if id == u64::MAX {
            return Err(api::ErrorAdd::MaxCapacity);
        }

        let product = api::Product {
            id,
            name: product.name,
        };

        products.push(Some(product));

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::bindings::exports::stan::product::api::Guest;
    use super::*;

    impl PartialEq for api::Product {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    #[test]
    fn get() {
        clear();

        let id = 0;
        let name = "foo".to_string();

        let product = api::ProductAdd { name: name.clone() };

        let _ = Component::add(product);
        let output = Component::get(id);

        let product = api::Product { id, name };

        assert_eq!(output, Some(product));
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

        let product = api::ProductAdd {
            name: "foo".to_string(),
        };

        let output = Component::add(product.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_incrementing_ids() {
        clear();

        let id = 1;

        let product = api::ProductAdd {
            name: "foo".to_string(),
        };

        let _ = Component::add(product.clone());
        let output = Component::add(product.clone());

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
        let mut products = PRODUCTS.lock().unwrap();
        products.clear();
        drop(products);
    }
}
