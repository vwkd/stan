cargo_component_bindings::generate!();

use crate::bindings::exports::golem::template::api;

struct Component;

impl api::Guest for Component {
    fn create(_routing_id: String, _amount: u32) -> Result<(), api::Error> {
        // get routing
        // let _routing = ...::routing::get(routing_id);

        // flatten tree to linear sequence

        // run sequence

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn create() {}
}
