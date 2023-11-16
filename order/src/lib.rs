cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;
use crate::bindings::golem::api::host::*;

struct Component;

impl Guest for Component {
    fn create(items: Vec<Item>) -> Result<(), Error> {
        // inventory
        let inventory_promise_id = golem_create_promise();
        golem_complete_promise(&inventory_promise_id, &[]);
        let _ = golem_await_promise(&inventory_promise_id);

        // ship
        // todo:

        // pay
        // todo:

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
