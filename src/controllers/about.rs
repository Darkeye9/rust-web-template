extern crate rustc_serialize;

use iron::{Request, Response, IronResult, status};
use router::Router;
use std::collections::BTreeMap;
use lib::ui;

pub fn register_routes(router: &mut Router, prefix: &str) {
    router.get(format!("{}{}", prefix, "about"), handle, "about");
}

#[allow(unused_variables)]
pub fn handle(req: &mut Request) -> IronResult<Response> {
    ui::render_view("About", "about", BTreeMap::new(), status::Ok)
}
