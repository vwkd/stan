mod worker;

cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;
use worker::{await_worker, Data};

struct Component;

impl Guest for Component {
    fn create(_items: Vec<Item>) -> Result<(), Error> {
        // inventory
        let _result = await_worker("inventory", Data {});

        // shipment
        let _result = await_worker("shipment", Data {});

        // payment
        let _result = await_worker("payment", Data {});

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
