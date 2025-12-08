use dap::responses::BreakpointLocationsResponse;
use dap::{prelude::*, types::Capabilities};
use std::io::{BufReader, BufWriter};

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use dap::events::OutputEventBody;
use dap::types::OutputEventCategory;

fn dap_log<S: std::io::Read, W: std::io::Write>(server: &mut Server<S, W>, msg: impl AsRef<str>) {
    let _ = server.send_event(Event::Output(OutputEventBody {
        category: Some(OutputEventCategory::Console),
        output: format!("{}\n", msg.as_ref()),
        ..Default::default()
    }));
    eprintln!("[DAP] {}", msg.as_ref());
}

fn main() -> DynResult<()> {
    eprintln!("[DAP] Starting Rust DAP Adapter...");

    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let mut server = Server::new(input, output);

    eprintln!("[DAP] Server created, waiting for requests...");

    loop {
        let req = match server.poll_request()? {
            Some(req) => req,
            None => {
                eprintln!("No request received, exiting.");
                break;
            }
        };

        match &req.command {
            Command::Initialize(args) => {
                dap_log(&mut server, format!("Initialize: {args:?}"));
                dap_log(&mut server, "Sending capabilities...");
                let rsp = req.success(ResponseBody::Initialize(Capabilities::default()));
                server.respond(rsp)?;
                server.send_event(Event::Initialized)?;
            }
            Command::Launch(args) => {
                dap_log(&mut server, format!("Launch: {args:?}"));
                server.respond(req.success(ResponseBody::Launch))?;
            }
            Command::Attach(args) => {
                dap_log(&mut server, format!("Attach: {args:?}"));
                server.respond(req.success(ResponseBody::Attach))?;
            }
            Command::BreakpointLocations(args) => {
                dap_log(&mut server, format!("BreakpointLocations: {args:?}"));
                server.respond(req.success(ResponseBody::BreakpointLocations(
                    BreakpointLocationsResponse {
                        breakpoints: vec![],
                    },
                )))?;
            }
            _ => {
                dap_log(&mut server, format!("Received: {:?}", req.command));
                server.respond(req.success(ResponseBody::Attach))?;
            }
        }
    }

    Ok(())
}
