pub mod config;
pub mod gh_decoder;
pub mod tests;

use config::Config;
use gh_decoder::ModListing;

struct FilterEntry<'a> {
    pub name: &'a str,
    pub author: &'a str,
}

impl<'a> From<&'a String> for FilterEntry<'a> {
    fn from(value: &'a String) -> Self {
        let Some((author, name)) = value.split_once('.') else {
            panic!("Invalid filter entry format");
        };
        FilterEntry { name, author }
    }
}

pub fn resolve_mod_filter<'a>(mod_list: &'a [ModListing], cfg: &Config) -> Vec<&'a ModListing> {
    // let filter_list: Vec<FilterEntry> = cfg.filter.list.iter().map(FilterEntry::from).collect();
    let filter_list: &[String] = &cfg.filter.list;
    (0..mod_list.len())
        .filter(|&i| {
            let entry = &mod_list[i];
            filter_list.iter().any(|s| s == entry.id())
        }) // iterator of all matching indices
        .map(|i| &mod_list[i])
        .collect()
}
