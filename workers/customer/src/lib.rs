cargo_component_bindings::generate!();
use bindings::exports::stan::customer::api;
use std::sync::Mutex;

static CUSTOMERS: Mutex<Vec<Option<api::Customer>>> = Mutex::new(Vec::new());

struct Component;

impl api::Guest for Component {
    fn get(id: api::Id) -> Option<api::Customer> {
        let id: usize = id.try_into().expect("64-bit OS");

        let customers = CUSTOMERS.lock().unwrap();

        customers.get(id).cloned().unwrap_or(None)
    }

    fn add(customer: api::CustomerAdd) -> Result<api::Id, api::ErrorAdd> {
        let mut customers = CUSTOMERS.lock().unwrap();

        let id = customers.len();
        let id: api::Id = id.try_into().expect("64-bit OS");

        if id == u64::MAX {
            return Err(api::ErrorAdd::MaxCapacity);
        }

        let customer = api::Customer {
            id,
            name: customer.name,
        };

        customers.push(Some(customer));

        Ok(id)
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::bindings::exports::stan::customer::api::Guest;
    use super::*;

    impl PartialEq for api::Customer {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    #[test]
    fn get() {
        clear();

        let id = 0;
        let name = "foo".to_string();

        let customer = api::CustomerAdd { name: name.clone() };

        let _ = Component::add(customer);
        let output = Component::get(id);

        let customer = api::Customer { id, name };

        assert_eq!(output, Some(customer));
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

        let customer = api::CustomerAdd {
            name: "foo".to_string(),
        };

        let output = Component::add(customer.clone());

        assert_eq!(output, Ok(id));
    }

    #[test]
    fn add_incrementing_ids() {
        clear();

        let id = 1;

        let customer = api::CustomerAdd {
            name: "foo".to_string(),
        };

        let _ = Component::add(customer.clone());
        let output = Component::add(customer.clone());

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
        let mut customers = CUSTOMERS.lock().unwrap();
        customers.clear();
        drop(customers);
    }
}
