cargo_component_bindings::generate!();
use bindings::exports::golem::order::api;
use bindings::golem::inventory;

struct Component;

impl api::Guest for Component {
    fn create(item: api::Item, payment: api::Payment) -> Result<(), api::Error> {
        // decrease inventory of item by quantity
        inventory::api::decrease(&item.product_id, item.quantity).map_err(|e| match e {
            inventory::api::Error::TooLow => api::Error::InventoryTooLow,
            _ => todo!(),
        })?;

        // handle payment
        let result = payment.handler(item.price).map_err(|e| match e {
            api::ErrorPayment::Declined => api::Error::PaymentDeclined,
            _ => todo!(),
        });

        if let Err(err) = result {
            // revert decrease inventory
            inventory::api::increase(&item.product_id, item.quantity).map_err(|e| match e {
                _ => todo!(),
            })?;

            return Err(api::Error::PaymentDeclined);
        }

        // notification of successful order or error

        // shipment
        // confirm shipment
        // todo: if error, revert previous steps

        // notification of successful shipment or error
        // notification of successful payment or error

        Ok(())

        // let payment_intent = create_payment_intent(user.id, order.total_amount, payment_method)?;

        // let payment = charge_payment(payment_intent)?;

        // if payment.status != "succeeded" {
        //     update_inventory(product.id, -order.quantity)?;
        //     return Err("Payment failed");
        // }

        // let fulfillment_order = dispatch_order(order, user)?;

        // send_order_confirmation(user.email, fulfillment_order)?;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
