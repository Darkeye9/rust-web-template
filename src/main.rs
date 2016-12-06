extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate urlencoded;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate regex;
extern crate clap;

#[macro_use]
extern crate lazy_static;

use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use hbs::{HandlebarsEngine, DirectorySource};

#[cfg(feature = "watch")]
use hbs::Watchable;

use std::path::Path;
use std::sync::Arc;

mod controllers;
mod middlewares;
mod utils;
mod lib;

#[cfg(feature = "watch")]
fn set_watch(hbse_ref: &Arc<HandlebarsEngine>) {
    hbse_ref.watch("./private/templates/");
}
#[cfg(not(feature = "watch"))]
#[allow(unused_variables)]
fn set_watch(hbse_ref: &Arc<HandlebarsEngine>) {}

fn main() {
    //Parse CLI arguments
    let cli_args = utils::cli::get_cli();

    //Load config
    lib::config::Config::init_config(cli_args.is_present("new_site"));

    //Startup perfomance monitoring
    utils::performance::PerfomanceCollector::init();

    //Set up the router
    let mut main_router = Router::new();

    //Register the routes
    controllers::about::register_routes(&mut main_router, "/");
    controllers::index::register_routes(&mut main_router, "/");

    //Set up the static routes
    let mut static_mount = Mount::new();
    static_mount.mount("/", main_router);
    static_mount.mount("/static", Static::new(Path::new("public/static/")));

    //Setup the templating engine
    let mut templating = HandlebarsEngine::new();
    templating.add(Box::new(DirectorySource::new("./private/templates/", ".tpl.html")));
    if let Err(r) = templating.reload() {
        panic!("{:?}", r);
    }

    let templating_ref = Arc::new(templating);
    set_watch(&templating_ref);

    //Create the processing chain
    let mut chain = Chain::new(static_mount);
    chain.link_before(middlewares::time::ResponseTime);
    chain.link_before(lib::error::ErrorHandler);

    chain.link_after(templating_ref);
    chain.link_after(middlewares::time::ResponseTime);
    chain.link_after(lib::error::ErrorHandler);

    //Create the server object
    let server = Iron::new(chain);


    println!("Server launched! Visit http://127.0.0.1:3000");
    server.http("127.0.0.1:3000").unwrap();
}
