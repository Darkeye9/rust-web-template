use rustc_serialize::json::{ToJson, Json};
use std::collections::{BTreeMap, HashMap};

#[derive(Default, Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct SiteConfig {
    pub site_name: String,
    pub published: bool,
}

impl ToJson for SiteConfig {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("site_name".to_string(), self.site_name.to_json());
        m.insert("published".to_string(), self.published.to_json());
        m.to_json()
    }
}

impl SiteConfig {
    #[allow(dead_code)]
    pub fn from_query(data: &HashMap<String, Vec<String>>) -> SiteConfig {
        let mut new_config = SiteConfig::default();

        if let Some(site_name) = data.get("site-name") {
            new_config.site_name = site_name.get(0).unwrap().clone();
        }
        if let Some(site_published) = data.get("published") {
            new_config.published = if site_published.get(0).unwrap().as_str() == "on" {
                true
            } else {
                false
            };
        } else {
            new_config.published = false;
        }

        new_config
    }
}
