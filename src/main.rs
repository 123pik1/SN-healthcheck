use crate::instances::ConfigFile;

mod client;
mod instances;
mod settings;

fn main() {
    #[allow(unused_must_use)]
    dotenvy::dotenv();

    let settings = settings::Settings::load_from_env();
    let config = ConfigFile::load_from_yaml(settings.config_path);
}
