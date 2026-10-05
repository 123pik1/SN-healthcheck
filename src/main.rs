use crate::instances::ConfigFile;

mod client;
mod instances;
mod settings;

fn main() {
    println!("Hello, world!");

    #[allow(unused_must_use)]
    dotenvy::dotenv();

    let settings = settings::Settings::load_from_env();
    let config = ConfigFile::load_from_yaml(settings.config_path);
    config.display_config();
}
