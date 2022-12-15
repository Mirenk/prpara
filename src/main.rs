use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli --pid [pid]")
        .action(print_arg)
        .flag(
            Flag::new("pid", FlagType::Int)
            .description("pid")
            .alias("p"),
            );

    app.run(args);
}

fn usage(c: &Context) {
    println!("Usage: cli --pid [pid]");
}

fn print_arg(c: &Context) {
    if let Ok(pid) = c.int_flag("pid"){
        println!("{}", pid);
    } else {
        usage(c);
    }
}
