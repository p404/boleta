// Chrono libs
extern crate chrono;
use chrono::prelude::*;

// Curl libs
extern crate curl_easybuilder;
use std::io::Write;
use curl_easybuilder::EasyBuilder;

// Read env variables 
use std::env;

// Check path
use std::path::Path;

// Write files libs
use std::fs::File;

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

    // Configuration
    let home_path = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => "/".to_string(),
    };

    let configuration_file_path = format!("{}/.boleta.yml", home_path);
    let path = Path::new(&configuration_file_path);
    let display = path.display();

    // Check if the file exists
    if path.exists() {
        println!("{} exists", display);
    }


    // Today's time 
    let today = Local::now();
    let today_parsed = today.format("%b %d, %Y").to_string();
    let today_last_month_day = last_day_of_month(today.year(), today.month());
    let today_last_month_day_parsed = UTC.ymd(today.year(), today.month(), today_last_month_day).format("%b %d, %Y").to_string();

    println!("{}", today_parsed );
    println!("{}", today_last_month_day_parsed );

    // Data creation
    let from = "Pablo";
    let to = "Enterprise";
    let job = "some job";
    let notes = "send to my bank account";
    let form = format!("from={}&\
                        to={}&\
                        number={}&\
                        date={}&\
                        due_date={}&\
                        items[0][name]={}&\
                        items[0][quantity]={}&\
                        items[0][unit_cost]={}&\
                        notes={}", 
                        from, to, 1, today_parsed, today_last_month_day_parsed, job, 10, 30, notes);


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