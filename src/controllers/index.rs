extern crate rustc_serialize;

use iron::{Request, Response, IronResult, status};
use std::collections::BTreeMap;
use router::Router;
use lib::ui;

pub fn register_routes(router: &mut Router, prefix: &str) {
    router.get(prefix, handle, "root");
}

#[allow(unused_variables)]
pub fn handle(req: &mut Request) -> IronResult<Response> {
    ui::render_view("Welcome", "front", BTreeMap::new(), status::Ok)
}
