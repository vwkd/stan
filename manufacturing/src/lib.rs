mod routing;

cargo_component_bindings::generate!();

use crate::bindings::exports::golem::template::api::*;
use routing::get_routing;

struct Component;

impl Guest for Component {
    fn create(routing_id: String, _amount: u32) -> Result<(), Error> {
        // get routing
        let _routing = get_routing(routing_id);

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
