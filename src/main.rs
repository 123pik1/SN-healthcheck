use std::env;

use crate::{client::Client, instances::ConfigFile};

mod client;
mod instances;
mod settings;

fn main() {
    #[allow(unused_must_use)]
    dotenvy::dotenv();

    let settings = settings::Settings::load_from_env();
    let config = ConfigFile::load_from_yaml(&settings.config_path);

    for instance in config.instances {
        let client = Client::init(
            env::var(instance.api_key)
                .unwrap_or(settings.sn_api_key.clone())
                .as_str(),
            env::var(instance.instance_url)
                .unwrap_or(settings.sn_instance_url.clone())
                .as_str(),
            env::var(instance.endpoint)
                .unwrap_or(settings.sn_api_url.clone())
                .as_str(),
            settings.clone(),
        );

        let response = client.get_stats();

        if let Some(value) = response.answer {
        } else {
            eprintln!(
                "There was {} tries and stats couldn't be got",
                response.times_requested
            );
            continue;
        }
    }
}
