// Invoice
use chrono::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, YamlEmitter};
use curl_easybuilder::EasyBuilder;


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

pub fn create(configuration_file_path: String){
    // Todays time 
    let today = Local::now();
    let today_parsed = today.format("%b %d, %Y").to_string();
    let today_last_month_day = last_day_of_month(today.year(), today.month());
    let today_last_month_day_parsed = UTC.ymd(today.year(), today.month(), today_last_month_day).format("%b %d, %Y").to_string();
     
    // Load data from YAML
    let mut configuration_file = File::open(&configuration_file_path).unwrap();
    let mut configuration_file_string = String::new();
    match configuration_file.read_to_string(&mut configuration_file_string) {
        Err(_)    => panic!("Couldn't read configuration file"),
        Ok(_)     => print!("Success!\n"),
    }
    let bill_load = YamlLoader::load_from_str(&configuration_file_string).unwrap();
    let bill = &bill_load[0];

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