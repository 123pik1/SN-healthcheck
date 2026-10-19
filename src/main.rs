use std::env;

use crate::{
    client::Client,
    instances::{CheckResult, ConfigFile},
};

mod client;
mod instances;
mod settings;

fn generate_report(results: Vec<CheckResult>) {
    let headers = ["CHECK", "DESCRIPTION", "STATUS"];
    let rows: Vec<[String; 3]> = results
        .into_iter()
        .map(|result| {
            [
                result.check_name,
                result.description,
                result.state.to_string(),
            ]
        })
        .collect();

    let widths = [
        rows.iter()
            .map(|row| row[0].len())
            .max()
            .unwrap_or(headers[0].len())
            .max(headers[0].len()),
        rows.iter()
            .map(|row| row[1].len())
            .max()
            .unwrap_or(headers[1].len())
            .max(headers[1].len()),
        rows.iter()
            .map(|row| row[2].len())
            .max()
            .unwrap_or(headers[2].len())
            .max(headers[2].len()),
    ];

    let separator = format!(
        "+-{}-+-{}-+-{}-+",
        "-".repeat(widths[0]),
        "-".repeat(widths[1]),
        "-".repeat(widths[2])
    );

    println!("{separator}");
    println!(
        "| {:<width0$} | {:<width1$} | {:<width2$} |",
        headers[0],
        headers[1],
        headers[2],
        width0 = widths[0],
        width1 = widths[1],
        width2 = widths[2],
    );
    println!("{separator}");

    for row in rows {
        println!(
            "| {:<width0$} | {:<width1$} | {:<width2$} |",
            row[0],
            row[1],
            row[2],
            width0 = widths[0],
            width1 = widths[1],
            width2 = widths[2],
        );
    }

    println!("{separator}");
}

fn main() {
    #[allow(unused_must_use)]
    dotenvy::dotenv();

    let settings = settings::Settings::load_from_env();
    let config = ConfigFile::load_from_yaml(&settings.config_path);

    for instance in config.instances {
        let client = Client::init(
            env::var(&instance.api_key)
                .unwrap_or(settings.sn_api_key.clone())
                .as_str(),
            env::var(&instance.instance_url)
                .unwrap_or(settings.sn_instance_url.clone())
                .as_str(),
            env::var(&instance.endpoint)
                .unwrap_or(settings.sn_api_url.clone())
                .as_str(),
            settings.clone(),
        );

        let response = client.get_stats();

        if let Some(value) = response.answer {
            let checks_results = instance.do_checks(value);
            generate_report(checks_results);
        } else {
            eprintln!(
                "There was {} tries and stats couldn't be got",
                response.times_requested
            );
            continue;
        }
    }
}
