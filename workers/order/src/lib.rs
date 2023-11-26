cargo_component_bindings::generate!();
use bindings::exports::stan::order::api;
use bindings::stan::inventory;

struct Component;

impl From<inventory::api::ErrorDecrease> for api::Error {
    fn from(value: inventory::api::ErrorDecrease) -> Self {
        match value {
            inventory::api::ErrorDecrease::TooLow => api::Error::InventoryTooLow,
        }
    }
}

impl api::Guest for Component {
    fn create(item: api::Item, _customer: api::Customer) -> Result<(), api::Error> {
        // todo: if error, revert previous steps

        // STATUS: pending
        // validate
        // try to decrease inventory
        inventory::api::decrease(item.product_id, item.quantity)?;
        // wait for approval

        // STATUS: processing
        // prepare shipping
        // let shipment_details = shipment::api::ship(item.product_id, item.quantity, customer.address)?;
        // notify customer, we confirm your order

        // STATUS: shipping
        // notify customer, your order is on the way

        // STATUS: fulfilled
        // invoice generation
        // financial transaction
        // notify customer, please pay us

        // STATUS: paid
        // payment

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
