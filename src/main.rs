
// Standard libraries
use std::io::Write;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;

// Chrono 
extern crate chrono;
use chrono::prelude::*;

// Curl 
extern crate curl_easybuilder;
use curl_easybuilder::EasyBuilder;

// YAML
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

// CLI
extern crate clap;
mod cli;
use cli::build_cli;

// Chrono functions
fn is_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => panic!("invalid month: {}" , month),
    }
}
 
fn main() {

    let option = cli::build_cli();
    let configure = option.value_of("configure").unwrap_or("default.conf");


    // Today's time 
    let today = Local::now();
    let today_parsed = today.format("%b %d, %Y").to_string();
    let today_last_month_day = last_day_of_month(today.year(), today.month());
    let today_last_month_day_parsed = UTC.ymd(today.year(), today.month(), today_last_month_day).format("%b %d, %Y").to_string();

    println!("{}", today_parsed );
    println!("{}", today_last_month_day_parsed );

    // Data creation
    // Load data from YAML
    let mut configuration_file = File::open(&configuration_file_path).unwrap();
    let mut configuration_file_string = String::new();
    match configuration_file.read_to_string(&mut configuration_file_string) {
        Err(why) => panic!("Couldn't read"),
        Ok(_) => print!("Success!"),
    }
    let bill_load = YamlLoader::load_from_str(&configuration_file_string).unwrap();
    let bill = &bill_load[0];
    
    
    // TODO Refactor with match
    let from         = bill["from"].as_str().unwrap();
    let to           = bill["bill-to"].as_str().unwrap();
    let job          = bill["service"].as_str().unwrap();
    let notes        = bill["notes"].as_str().unwrap();
    let hours        = bill["hours"].as_i64().unwrap();
    let cost         = bill["cost-per-hour"].as_i64().unwrap();
    let number       = bill["last-invoice-number"].as_i64().unwrap();
    let form         = format!("from={}&\
                                to={}&\
                                number={}&\
                                date={}&\
                                due_date={}&\
                                items[0][name]={}&\
                                items[0][quantity]={}&\
                                items[0][unit_cost]={}&\
                                notes={}", 
                                from, to, number, today_parsed, today_last_month_day_parsed, job, hours, cost, notes);

   
    println!("{:?}", form );


    // Invoice request/file creation
    let data_form = form.as_bytes();
    let mut file = File::create("Invoice.pdf").unwrap();
    let mut easy = EasyBuilder::new();
    let mut buf = Vec::new();

    let easy = easy.url("https://invoice-generator.com")
        .post(true)
        .post_fields_copy(data_form)
        .write_function(move |data|{
            buf.extend_from_slice(data);
            file.write_all(&buf).unwrap();
            Ok(data.len())
        })
        .result()
        .unwrap();
    easy.perform().unwrap();
}