use clap::{Arg, App, SubCommand};

pub fn build_cli() {
            App::new("Boleta")
            .author("Pablo Opazo <paocamd@msn.com>")
            .about("CLI For creating invoices")
            .version("0.1.0")
            .arg(Arg::with_name("configure")
                               .short("c")
                               .long("configure")
                               .help("Sets configuration for invoices")
                               .takes_value(false)
                               .required(false))
            .arg(Arg::with_name("new")
                                .short("n")
                                .long("new")
                                .help("Creates a new invoice")
                                .takes_value(false)
                                .required(false))
            .get_matches();

}
