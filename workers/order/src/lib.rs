cargo_component_bindings::generate!();
use bindings::exports::stan::order::api;
use bindings::stan::inventory;

struct Component;

impl api::Guest for Component {
    fn create(item: api::Item) -> Result<(), api::Error> {
        // decrease inventory of item by quantity
        inventory::api::decrease(&item.product_id, item.quantity).map_err(|e| match e {
            inventory::api::Error::TooLow => api::Error::InventoryTooLow,
            _ => todo!(),
        })?;

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
