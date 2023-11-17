mod worker;

cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;
use worker::{await_worker, Data};

struct Component;

impl Guest for Component {
    fn create(_items: Vec<Item>) -> Result<(), Error> {
        // inventory
        // decrease each item by quantity
        let _result = await_worker("inventory", Data {});

        // notification of successful order or error

        // shipment
        // confirm shipment
        let _result = await_worker("shipment", Data {});
        // todo: if error, revert previous steps

        // notification of successful shipment or error

        // payment
        // send payment
        let _result = await_worker("payment", Data {});
        // todo: if error, revert previous steps

        // notification of successful payment or error

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
