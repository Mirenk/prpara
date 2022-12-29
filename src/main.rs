use seahorse::{App, Context, Flag, FlagType};
use std::env;
use ptrace_rust::ptrace::Proc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli --pid [pid]")
        .action(run)
        .flag(
            Flag::new("pid", FlagType::Int)
            .description("pid")
            .alias("p"),
            );

    app.run(args);
}

fn usage() {
    println!("Usage: cli --pid [pid]");
}

fn run(c: &Context) {
    if let Ok(pid) = c.int_flag("pid"){
        if pid > 0 {
            let proc = Proc::new(pid.try_into().unwrap());
        } else {
            eprintln!("error: pid must positive number.");
        }
    } else {
        usage();
    }
}
