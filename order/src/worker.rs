use crate::bindings::golem::api::host::*;

pub struct Data {}

pub enum Error {
    // todo: ...
}

/// Await Golem worker
///
/// - call other worker with promise_id and input data via HTTP
/// - wait until other worker completed promise with output data
///
/// ## Arguments
/// - `name`: name of template and worker with suffix `-1`
/// - `data`: input data to send to worker
///
/// todo: workaround for missing direct API, replace with direct call once API exists
pub fn await_worker(_name: &str, _data: Data) -> Result<Data, Error> {
    let promise_id = golem_create_promise();

    // todo: encode data into &[u8]

    // todo: send promise_id and data to worker via HTTP, which URL, which endpoint?
    // todo: move to that worker
    golem_complete_promise(&promise_id, &[]);

    let _result = golem_await_promise(&promise_id);

    // todo: decode data from Vec<u8>

    Ok(Data {})
}
