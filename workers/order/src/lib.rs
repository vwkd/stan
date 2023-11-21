cargo_component_bindings::generate!();
use bindings::exports::golem::order::api;
use bindings::golem::inventory;

struct Component;

impl api::Guest for Component {
    fn create(item: api::Item) -> Result<(), api::Error> {
        // inventory
        // decrease item by quantity
        let _result = inventory::api::decrease(&item.product_id, item.quantity);

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
