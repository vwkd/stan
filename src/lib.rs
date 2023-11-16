cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct State {}

static STATE: Mutex<State> = Mutex::new(State {});

struct Component;

impl Guest for Component {
    fn get() -> () {}
}

#[cfg(test)]
// beware: must run sequentially with `cargo test -- --test-threads=1`
mod tests {
    use super::*;

    #[test]
    fn get() {}
}
