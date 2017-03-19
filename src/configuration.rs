pub fn setup() {

    let home_path = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => "/".to_string(),
    };

    let configuration_file_path = format!("{}/.boleta.yml", home_path);
    let path = Path::new(&configuration_file_path);
    let display = path.display();

    // Check if the file exists
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