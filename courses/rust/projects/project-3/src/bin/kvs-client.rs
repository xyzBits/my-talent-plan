use std::net::SocketAddr;

use structopt::StructOpt;
use clap::AppSettings;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:40000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs-client",
    raw(global_settings = "&[\
    AppSettings::DisableHelpSubcommand,\
    AppSettings::VersionlessSubcommands]")
)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}


#[derive(StructOpt, Debug)]
enum Command {

    #[structopt(name = "get", about = "Get the string value of a given string key")]
    Get {

        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT")
            raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },

    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    Set {

        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(name = "VALUE", help = "The string value of the key")]
        value: String,

        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT")
            raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },

    #[structopt(name = "rm", about = "Remove a given string key")]
    Remove {

        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT")
            raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}


fn main() {}