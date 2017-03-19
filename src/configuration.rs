// Standard libraries
use std::env;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Write;

fn confirmation() -> bool{
    println!("Do you want to update your current configuration? (y/n)");
    let mut question = String::new();
        io::stdin().read_line(&mut question).ok();
        match question[..].trim() {
            "Y" | "y" | "yes" | "YES" => return true,
            _ => return false,
        }
}
fn configuration_bootstrap(path: &Path) {
// This need refactor; read! implementation   
    println!("From:");
    let from: String = read!();

    println!("Bill to:");
    let bill_to: String = read!();

    println!("Last invoice number:");
    let number: String = read!();

    println!("Service:");
    let job: String = read!();

    println!("Hours:");
    let hours: String = read!();

    println!("Cost per hour:");
    let cost: String = read!();

    println!("Notes:");
    let notes: String = read!();

    println!("Invoices folder path");
    let invoice_folder_path: String = read!();

    let data = format!("from: {} 
bill-to: {}
last-invoice-numberumber: {}
service: {}
hours: {}
cost-per-hour: {}
notes: {} 
invoice_folder_path: {}", from, bill_to, number, job, hours, cost, notes, invoice_folder_path);

    let mut configuration_file = File::create(path).unwrap();
    configuration_file.write_all(data.as_bytes());
}

pub fn setup(configuration_file_path: String) {
    let path = Path::new(&configuration_file_path);
    let display = path.display();

    if path.exists() {
        println!("The configuration file {} already exists", display);
        if confirmation() == true {
            configuration_bootstrap(path);
        } else {
            panic!("Oh no!");
        }
    } else {
        println!("Setup new invoice configuration");
        configuration_bootstrap(path);
    }
}
