use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;

use clap::arg_enum;
use log::{debug, error, info, Level, LevelFilter, log_enabled, warn};
use structopt::StructOpt;

use kvs::{KvsEngine, KvsServer, KvStore, Result, SledKvsEngine};


const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

// StructOpt: This derive comes from the structopt crate and enables paring
// command-line arguments based on the structure's fields.


#[derive(StructOpt, Debug)]
// This attribute specifies the name of the program for which these arguments are intended.
// Here, it's set to "kvs-server". This name will be used in help messages and documentation.
#[structopt(name = "kvs-server")]
struct Opt {
    #[structopt(
        long,
        help = "Sets the listening address",
        value_name = "IP:PORT",
        raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
        parse(try_from_str)
    )]
    addr: SocketAddr,

    #[structopt(
        long,
        help = "Sets the storage engine",
        value_name = "ENGINE-NAME",
        raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,

}

// This macro is provided by the clap crate for defining enumerations that can be used as arguments for command-line tools.
arg_enum! {
    // this attribute suppresses a potential Rust warning about using non-camel case for the enum name
    // In Rust, convention dictates using camel case for type name
    // however kvs and sled might be appropriate abbreviations in this context
    #[allow(non_camel_case_types)]
    // Debug: enables easy printing of the enum variant using {:?}
    // Copy: make the enum a copy type
    // Clone: enables cloning the enum variant.
    // Partialeq: allows comparison for equality using the == operator (e.g., engine_type1 == Engine::kvs).
    // Eq: allows comparison for strict equality using the eq function
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        kvs,
        sled
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // Gets the struct from the command line arguments.
    // Print the error message and quit the program in case of failure
    let mut opt = Opt::from_args();
    info!("start kvs-server opt: {:?}", opt);

    let res = current_engine()
        .and_then(move |curr_engine| {
            if opt.engine.is_none() {
                opt.engine = curr_engine;
            }

            if curr_engine.is_some() && opt.engine != curr_engine {// != supported by PartialEq, Eq
                error!("Wrong engine!");
                exit(1);
            }

            run(opt)
        });

    if let Err(e) = res {
        error!("{}", e);
        exit(1);
    }
}


fn run(opt: Opt) -> Result<()> {
    let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", opt.addr);

    // write engine to engine file
    fs::write(current_dir()?.join("engine"), format!("{}", engine))?;

    match engine {
        Engine::kvs => { run_with_engine(KvStore::open(current_dir()?)?, opt.addr) }
        Engine::sled => { run_with_engine(SledKvsEngine::new(sled::open(current_dir()?)?), opt.addr) }
    }
}

fn run_with_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = KvsServer::new(engine);
    server.run(addr)
}


fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => {
            Ok(Some(engine))
        }

        Err(e) => {
            warn!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}


#[test]
fn test_env_logger() {
    // A simple logger configured via environment variables which writes to stdout or stderr,
    // for use with the logging facade exposed by the log crate

    env_logger::init();
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4;
        info!("the answer was: {}", x);
    }
}