use dap::responses::SetBreakpointsResponse;
use dap::{prelude::*, types::Capabilities};
use std::io::{BufReader, BufWriter};

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use dap::events::OutputEventBody;
use dap::types::{Breakpoint, OutputEventCategory};

fn dap_log<S: std::io::Read, W: std::io::Write>(server: &mut Server<S, W>, msg: impl AsRef<str>) {
    let _ = server.send_event(Event::Output(OutputEventBody {
        category: Some(OutputEventCategory::Console),
        output: format!("{}\n", msg.as_ref()),
        ..Default::default()
    }));
}

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

        dap_log(
            &mut server,
            format!("Processing command: {:?}", req.command),
        );

        let result: DynResult<()> = match &req.command {
            Command::Initialize(args) => {
                dap_log(&mut server, format!("Initialize: {args:?}"));
                let rsp = req.success(ResponseBody::Initialize(Capabilities::default()));
                server.respond(rsp)?;
                server.send_event(Event::Initialized)?;
                Ok(())
            }
            Command::Launch(args) => {
                dap_log(&mut server, format!("Launch: {args:?}"));
                server.respond(req.success(ResponseBody::Launch))?;
                Ok(())
            }
            Command::Attach(args) => {
                dap_log(&mut server, format!("Attach: {args:?}"));
                server.respond(req.success(ResponseBody::Attach))?;
                Ok(())
            }
            Command::SetBreakpoints(args) => {
                dap_log(&mut server, format!("SetBreakpoints: {args:?}"));
                let mut breakpoints = Vec::new();
                if let Some(source_breakpoints) = &args.breakpoints {
                    for (i, src_bp) in source_breakpoints.iter().enumerate() {
                        let breakpoint = Breakpoint {
                            id: Some(i as i64 + 1),
                            verified: true,
                            message: None,
                            source: Some(args.source.clone()),
                            line: Some(src_bp.line),
                            column: src_bp.column,
                            end_line: None,
                            end_column: None,
                            instruction_reference: None,
                            offset: None,
                        };
                        breakpoints.push(breakpoint);

                        dap_log(
                            &mut server,
                            format!("Set breakpoint at line {}", src_bp.line),
                        );
                        dap_log(&mut server, "Yo. Breakpoint set!");
                    }
                }

                server.respond(req.success(ResponseBody::SetBreakpoints(
                    SetBreakpointsResponse { breakpoints },
                )))?;
                Ok(())
            }
            _ => {
                dap_log(&mut server, format!("Unhandled command: {:?}", req.command));
                server.respond(req.success(ResponseBody::Launch))?;
                Ok(())
            }
        };

        if let Err(e) = result {
            eprintln!("[DAP] Error processing command: {}", e);
            dap_log(&mut server, format!("Error: {}", e));
        }
    }

    Ok(())
}
