#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

use std::process;

use rusty_dns_server::buffer::{BytePacketBuffer, Deserialize, Result};
use rusty_dns_server::dns::Message;

fn parse_response_packet_file(file_path: &str) -> Result<()> {
    let mut buf = BytePacketBuffer::new();
    buf.fill_from_file(file_path)?;

    let message = Message::deserialize(&mut buf)?;
    println!("{:#?}", message.header);
    for q in message.questions {
        println!("{:#?}", q);
    }
    for a in message.answers {
        println!("{:#?}", a);
    }
    for auth in message.authorities {
        println!("{:#?}", auth);
    }
    for adtl in message.additionals {
        println!("{:#?}", adtl);
    }

    Ok(())
}

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if let Some(stub) = matches.subcommand_matches("stub") {
        let path = stub.value_of("packet-file").unwrap();
        if let Err(e) = parse_response_packet_file(path) {
            eprintln!("Application error: {:#?}", e);
            process::exit(2);
        }
    }
}
