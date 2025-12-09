use std::io::{Stdin, Stdout};

use dap::events::Event;
use dap::requests::{
    AttachRequestArguments, Command, DisconnectArguments, InitializeArguments,
    LaunchRequestArguments, Request, RestartArguments, SetBreakpointsArguments,
    SetExceptionBreakpointsArguments,
};
use dap::responses::{
    ResponseBody, SetBreakpointsResponse, SetExceptionBreakpointsResponse, ThreadsResponse,
};
use dap::server::Server;
use dap::types::{Breakpoint, Thread};

use crate::log::dap_log;
use crate::types::DynResult;
use crate::utils::extract_port_from_args;

pub(crate) fn handle(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    match &req.command {
        Command::Initialize(args) => handle_initialize(req.clone(), args, server),
        Command::Launch(args) => handle_launch(req.clone(), args, server),
        Command::Restart(args) => handle_restart(req.clone(), args, server),
        Command::Attach(args) => handle_attach(req.clone(), args, server),
        Command::SetBreakpoints(args) => handle_set_breakpoints(req.clone(), args, server),
        Command::SetExceptionBreakpoints(args) => {
            handle_set_exception_breakpoints(req.clone(), args, server)
        }
        Command::Threads => handle_threads(req.clone(), server),
        Command::Disconnect(args) => handle_disconnect(req.clone(), args, server),
        _ => handle_unhandled_command(req, server),
    }
}

fn handle_initialize(
    req: Request,
    args: &InitializeArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Initialize: {args:?}"));
    let rsp = req.success(ResponseBody::ConfigurationDone);
    server.respond(rsp)?;
    server.send_event(Event::Initialized)?;
    Ok(())
}

fn handle_launch(
    req: Request,
    args: &LaunchRequestArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Launch: {args:?}"));
    let port = extract_port_from_args(args);

    dap_log(server, format!("Running on port: {port:?}"));
    server.respond(req.success(ResponseBody::Launch))?;
    Ok(())
}

fn handle_restart(
    req: Request,
    args: &RestartArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Restart: {args:?}"));
    server.respond(req.success(ResponseBody::Restart))?;
    Ok(())
}

fn handle_attach(
    req: Request,
    args: &AttachRequestArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Attach: {args:?}"));
    server.respond(req.success(ResponseBody::Attach))?;
    Ok(())
}

fn handle_set_breakpoints(
    req: Request,
    args: &SetBreakpointsArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
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

    server.respond(
        req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse {
            breakpoints,
        })),
    )?;
    Ok(())
}

fn handle_set_exception_breakpoints(
    req: Request,
    args: &SetExceptionBreakpointsArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("SetExceptionBreakpoints: {args:?}"));

    // This is a placeholder implementation.
    server.respond(req.success(ResponseBody::SetExceptionBreakpoints(
        SetExceptionBreakpointsResponse { breakpoints: None },
    )))?;
    Ok(())
}

fn handle_disconnect(
    req: Request,
    args: &DisconnectArguments,
    server: &mut Server<Stdin, Stdout>,
) -> DynResult<()> {
    dap_log(server, format!("Disconnect: {args:?}"));
    // Handle termination of the debuggee if specified
    if let Some(terminate) = args.terminate_debuggee {
        if terminate {
            dap_log(server, "Terminating debuggee as requested");
        } else {
            dap_log(server, "Keeping debuggee alive as requested");
        }
    }

    if let Some(restart) = args.restart {
        if restart {
            dap_log(server, "Disconnect is part of restart sequence");
        }
    }

    server.respond(req.success(ResponseBody::Disconnect))?;
    Ok(())
}

fn handle_threads(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    dap_log(server, "Threads request received");

    //This is a placeholder implementation.
    let threads = vec![Thread {
        id: 1,
        name: "Main Thread".to_string(),
    }];

    server.respond(req.success(ResponseBody::Threads(ThreadsResponse { threads })))?;
    Ok(())
}

fn handle_unhandled_command(req: Request, server: &mut Server<Stdin, Stdout>) -> DynResult<()> {
    dap_log(server, format!("Unhandled command: {:?}", req.command));
    server.respond(req.success(ResponseBody::Launch))?;
    Ok(())
}
