#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::{
        config::Config,
        gh_decoder::{self, ModListing},
        resolve_mod_filter,
    };

    #[tokio::test]
    async fn parse_all_mods() {
        let data: String = gh_decoder::get_raw_gh_data().await;
        let mods: Vec<ModListing> = gh_decoder::parse_gh_data(&data);
        dbg!(&mods[11]);
    }

    #[tokio::test]
    async fn find_specific_mod() {
        let data: String = gh_decoder::get_raw_gh_data().await;
        let mods: Vec<ModListing> = gh_decoder::parse_gh_data(&data);
        let result: Option<&ModListing> = mods.iter().find(|m| m.name() == "Convenient Inventory");
        dbg!(result);
    }

    #[tokio::test]
    async fn filter_resolution() {
        let data: String = gh_decoder::get_raw_gh_data().await;
        let mods: Vec<ModListing> = gh_decoder::parse_gh_data(&data);
        let cfg: Config = crate::config::parse_config(include_str!("../seedbox.toml")).unwrap();
        println!("using filter list {:?}", &cfg.filter.list);
        let filter: Vec<&ModListing> = resolve_mod_filter(&mods, &cfg);
        dbg!(filter);
    }
}
