cargo_component_bindings::generate!();
use crate::bindings::exports::golem::order::api;

struct Component;

impl api::Guest for Component {
    fn create(_item: api::Item) -> Result<(), api::Error> {
        // inventory
        // decrease item by quantity
        // let _result = ...::inventory::decrease(id: ..., amount: ...);

        // notification of successful order or error

        // shipment
        // confirm shipment
        // todo: if error, revert previous steps

        // notification of successful shipment or error

        // payment
        // send payment
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
