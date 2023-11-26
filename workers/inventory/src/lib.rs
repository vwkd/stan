cargo_component_bindings::generate!();
use bindings::exports::stan::inventory::api;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

// todo: verify id is of existing Product
static INVENTORY: Lazy<Mutex<HashMap<api::Id, api::Amount>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl api::Guest for Component {
    fn get(id: api::Id) -> api::Amount {
        INVENTORY.lock().unwrap().get(&id).cloned().unwrap_or(0)
    }

    fn increase(id: api::Id, amount: api::Amount) -> Result<(), api::ErrorIncrease> {
        let mut inventory = INVENTORY.lock().unwrap();

        let existing = inventory.get(&id).cloned().unwrap_or(0);

        if existing > api::Amount::MAX - amount {
            return Err(api::ErrorIncrease::TooHigh);
        }

        inventory.insert(id, existing + amount);

        Ok(())
    }

    fn decrease(id: api::Id, amount: api::Amount) -> Result<(), api::ErrorDecrease> {
        let mut inventory = INVENTORY.lock().unwrap();

        let existing = inventory.get(&id).cloned().unwrap_or(0);

        if amount > existing {
            return Err(api::ErrorDecrease::TooLow);
        }

        inventory.insert(id, existing - amount);

        Ok(())
    }
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::bindings::exports::stan::inventory::api::Guest;
    use super::*;

    #[test]
    fn get() {
        clear();

        let id = 123;
        let amount = 42;

        let _ = Component::increase(id, amount);
        let output = Component::get(id);

        assert_eq!(output, amount);
    }

    #[test]
    fn get_invalid() {
        clear();

        let output = Component::get(123);

        assert_eq!(output, 0);
    }

    #[test]
    fn increase() {
        clear();

        let id = 123;
        let amount = 42;

        let output = Component::increase(id, amount);

        assert_eq!(output, Ok(()));
    }

    #[test]
    fn increase_too_high() {
        clear();

        let id = 123;
        let amount = 42;

        let _ = Component::increase(id, api::Amount::MAX - 10);
        let output = Component::increase(id, amount);

        assert_eq!(output, Err(api::ErrorIncrease::TooHigh));
    }

    #[test]
    fn decrease() {
        clear();

        let id = 123;
        let amount = 42;

        let _ = Component::increase(id, 50);
        let output = Component::decrease(id, amount);

        assert_eq!(output, Ok(()));
    }

    #[test]
    fn decrease_too_low() {
        clear();

        let id = 123;
        let amount = 42;

        let output = Component::decrease(id, amount);

        assert_eq!(output, Err(api::ErrorDecrease::TooLow));
    }

    fn clear() {
        let mut inventory = INVENTORY.lock().unwrap();
        inventory.clear();
        drop(inventory);
    }
}
