mod command_handler;
mod log;
mod types;
mod utils;
use crate::command_handler::handle;
use crate::log::dap_log;
use crate::types::DynResult;
use dap::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() -> DynResult<()> {
    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let mut server = Server::new(input, output);

    loop {
        let req = match server.poll_request()? {
            Some(req) => req,
            None => {
                eprintln!("No request received, exiting.");
                break;
            }
        };

        let result: DynResult<()> = handle(req, &mut server);

        if let Err(e) = result {
            eprintln!("[DAP] Error processing command: {}", e);
            dap_log(&mut server, format!("Error: {}", e));
        }
    }

    Ok(())
}
