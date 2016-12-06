use hbs::Template;
use rustc_serialize::json::{ToJson, Json};
use iron::prelude::*;
use iron::status;
use std::collections::BTreeMap;
use lib::config;

pub fn render_view(title: &str,
                   template: &str,
                   mut in_data: BTreeMap<String, Json>,
                   status: status::Status)
                   -> IronResult<Response> {
    let mut resp = Response::new();

    // Get actual config
    let config = config::Config::get_site_config().unwrap();

    in_data.insert("title".to_string(),
                   format!("{} | {}", title, config.site_name).to_json());
    resp.set_mut(Template::new(template, in_data)).set_mut(status);
    Ok(resp)
}
