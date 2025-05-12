#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        config::{Config, load_config_from_wd},
        gh_decoder::{self, ModListing},
        pipeline::csproj_parser::insert_game_path_buf,
        resolve_mod_filter,
    };
    // use crate::pipeline::csproj_parser::tokenize_csproj;

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

    #[test]
    fn read_config() {
        let cfg: Option<Config> = load_config_from_wd();
        dbg!(cfg);
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

    #[test]
    fn insert_to_csproj() {
        let game_path: PathBuf = [
            "Users",
            "tahirchaudhry",
            "Library",
            "Application Support",
            "Steam",
            "steamapps",
            "common",
            "Stardew Valley",
            "Contents",
            "MacOS",
        ]
        .iter()
        .collect(); // possibly the ugliest way to build a path, but i have spaces :(

        let mut buffer: String = std::fs::read_to_string("./ConvenientInventory.csproj").unwrap();
        insert_game_path_buf(&mut buffer, &game_path);
        std::fs::write("./test.csproj", buffer.as_bytes()).unwrap();
    }

    // #[test]
    // fn csproj_tokens() {
    //     let input: &str = include_str!("../ConvenientInventory.csproj");
    //     let tokens: Vec<XmlToken> = tokenize_csproj(input);
    //     dbg!(tokens);
    // }
}
