// Standard libraries
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
    println!("From: (e.g. John Doe)");
    let from: String = read!("{}\n");

    println!("Bill to: (e.g. Enterprise, Inc)");
    let bill_to: String = read!("{}\n");

    println!("Last invoice number: (e.g. 2)");
    let last_invoice_number: String = read!();

    println!("Service: (e.g. Software engineer)");
    let job: String = read!("{}\n");

    println!("Hours: (e.g. 160)");
    let hours: String = read!("{}\n");

    println!("Cost per hour: (e.g. 20.00)");
    let cost: String = read!("{}\n");

    println!("Notes: (e.g. Please make a direct deposit)");
    let notes: String = read!("{}\n");

    println!("Invoices folder path (e.g. /home/john/)");
    let invoice_folder_path: String = read!("{}\n");

    let data = format!("from: {} 
bill-to: {}
last-invoice-number: {}
service: {}
hours: {}
cost-per-hour: {}
notes: {} 
invoice-folder-path: {}", from, bill_to, last_invoice_number, job, hours, cost, notes, invoice_folder_path);

    let mut configuration_file = File::create(path).unwrap();
    configuration_file.write_all(data.as_bytes()).unwrap();
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
