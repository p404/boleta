// Standard libraries
use std::env;

// CLI
extern crate clap;
use clap::{App, SubCommand};

// Configuration
mod configuration;
#[allow(unused_imports)]
use configuration::setup;

// Invoice
extern crate chrono;
extern crate yaml_rust;
extern crate curl_easybuilder;
mod invoice;
#[allow(unused_imports)]
use invoice::create;

fn configuration_path() -> String {
    let home_path = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => "/".to_string(),
    };
    format!("{}/.boleta.yml", home_path)
}

fn main() {
    let options = App::new("Boleta")
                    .author("Pablo Opazo <paocamd@msn.com>")
                    .about("CLI For creating invoices")
                    .version("0.1.0")
                    .subcommand(SubCommand::with_name("configure").about("Sets configuration for invoices"))
                    .subcommand(SubCommand::with_name("new").about("Creats a new invoice"))
                    .get_matches();

    let configuration_file_path = configuration_path().to_string();
  
    match options.subcommand() {
        ("configure", Some(_)) => configuration::setup(configuration_file_path),
        ("new", Some(_)) => invoice::create(configuration_file_path),
        _ => (),
    }
}