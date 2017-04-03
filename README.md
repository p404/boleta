# Boleta [![Build Status](https://travis-ci.org/p404/boleta.svg?branch=master)](https://travis-ci.org/p404/boleta)

Boleta is a small CLI application for generating invoices. 

Boleta stores your invoice configuration `$HOME/.boleta.yml` to generate new invoices.  

Binaries can be found under [releases](https://github.com/p404/boleta/releases)

### Installing
```bash
curl -sL https://raw.githubusercontent.com/p404/boleta/master/install.sh | bash
```

### Usage
```bash
Boleta 0.1.0
Pablo Opazo <pablo@sequel.ninja>
CLI For creating invoices

USAGE:
    boleta [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    configure    Sets configuration for invoices
    help         Prints this message or the help of the given subcommand(s)
    new          Creats a new invoice
```

