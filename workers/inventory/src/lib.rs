cargo_component_bindings::generate!();
use bindings::exports::stan::inventory::api;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

// todo: verify id is of existing Product
static INVENTORY: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct Component;

impl api::Guest for Component {
    fn get(id: String) -> u32 {
        INVENTORY.lock().unwrap().get(&id).cloned().unwrap_or(0)
    }

    fn increase(id: String, amount: u32) -> Result<(), api::Error> {
        let mut inventory = INVENTORY.lock().unwrap();

        let existing = inventory.get(&id).cloned().unwrap_or(0);

        if existing > u32::MAX - amount {
            return Err(api::Error::TooHigh);
        }

        inventory.insert(id, existing + amount);

        Ok(())
    }

    fn decrease(id: String, amount: u32) -> Result<(), api::Error> {
        let mut inventory = INVENTORY.lock().unwrap();

        let existing = inventory.get(&id).cloned().unwrap_or(0);

        if amount > existing {
            return Err(api::Error::TooLow);
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

        let id = "123".to_string();
        let amount = 42;

        let _ = Component::increase(id.clone(), amount);
        let output = Component::get(id.clone());

        assert_eq!(output, amount);
    }

    #[test]
    fn get_invalid() {
        clear();

        let output = Component::get("123".to_string());

        assert_eq!(output, 0);
    }

    #[test]
    fn increase() {
        clear();

        let id = "123".to_string();
        let amount = 42;

        let output = Component::increase(id.clone(), amount);

        assert_eq!(output, Ok(()));
    }

    #[test]
    fn increase_too_high() {
        clear();

        let id = "123".to_string();
        let amount = 42;

        let _ = Component::increase(id.clone(), u32::MAX - 10);
        let output = Component::increase(id.clone(), amount);

        assert_eq!(output, Err(api::Error::TooHigh));
    }

    #[test]
    fn decrease() {
        clear();

        let id = "123".to_string();
        let amount = 42;

        let _ = Component::increase(id.clone(), 50);
        let output = Component::decrease(id.clone(), amount);

        assert_eq!(output, Ok(()));
    }

    #[test]
    fn decrease_too_low() {
        clear();

        let id = "123".to_string();
        let amount = 42;

        let output = Component::decrease(id.clone(), amount);

        assert_eq!(output, Err(api::Error::TooLow));
    }

    fn clear() {
        let mut inventory = INVENTORY.lock().unwrap();
        inventory.clear();
        drop(inventory);
    }
}
