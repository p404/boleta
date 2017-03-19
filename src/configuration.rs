// Standard libraries
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn setup(configuration_file_path: String) {
    let path = Path::new(&configuration_file_path);
    let display = path.display();

    if path.exists() {
        println!("The configuration file {} already exists", display);
    } else {
        let mut configuration_file = File::create(path).unwrap();
        let configuration_bootstrap = "from: pablo
bill-to: company
service: contractor for some company
notes: send me more money
hours: 140
cost-per-hour: 30
last-invoice-number: 20
invoice-folder-path: /home/pablo/invoices";

        configuration_file.write_all(configuration_bootstrap.as_bytes());
    }

}