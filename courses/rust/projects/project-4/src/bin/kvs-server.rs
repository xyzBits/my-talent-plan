use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;

use clap::arg_enum;
use log::{error, LevelFilter, warn};
use structopt::StructOpt;
use walkdir::err;

use kvs::*;
use kvs::thread_pool::ThreadPool;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(StructOpt, Debug)]
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

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Engine {
        kvs,
        sled
    }
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let mut opt = Opt::from_args();
    let res = current_engine().and_then(move |curr_engine| {

        if opt.engine.is_none() {
            opt.engine = curr_engine;
        }

        if curr_engine.is_some() && opt.engine != curr_engine {
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
    todo!()
}

pub fn run_with<E: KvsEngine, P: ThreadPool>(engine: E, pool: P, addr: SocketAddr) -> Result<()> {
    todo!()
}

fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            warn!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}







