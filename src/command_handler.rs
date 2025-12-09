use std::io::{Stdin, Stdout};

use dap::events::Event;
use dap::requests::{Command, Request};
use dap::responses::{ResponseBody, SetBreakpointsResponse};
use dap::server::Server;
use dap::types::{Breakpoint, Capabilities};

use crate::log::dap_log;
use crate::types::DynResult;

pub(crate) fn handle(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    let result =
        match &req.command {
            Command::Initialize(args) => {
                dap_log(server, format!("Initialize: {args:?}"));
                let rsp = req.success(ResponseBody::Initialize(Capabilities::default()));
                server.respond(rsp)?;
                server.send_event(Event::Initialized)?;
                Ok(())
            }
            Command::Launch(args) => {
                dap_log(server, format!("Launch: {args:?}"));
                server.respond(req.success(ResponseBody::Launch))?;
                Ok(())
            }
            Command::Attach(args) => {
                dap_log(server, format!("Attach: {args:?}"));
                server.respond(req.success(ResponseBody::Attach))?;
                Ok(())
            }
            Command::SetBreakpoints(args) => {
                dap_log(server, format!("SetBreakpoints: {args:?}"));
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

                        dap_log(server, format!("Set breakpoint at line {}", src_bp.line));
                    }
                }

                server.respond(req.success(ResponseBody::SetBreakpoints(
                    SetBreakpointsResponse { breakpoints },
                )))?;
                Ok(())
            }
            _ => {
                dap_log(server, format!("Unhandled command: {:?}", req.command));
                server.respond(req.success(ResponseBody::Launch))?;
                Ok(())
            }
        };
    result
}
