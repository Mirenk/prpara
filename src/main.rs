use prpara::core::load;
use prpara::core::Proc;
use seahorse::{App, Context, Flag, FlagType};
use std::env;

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
        )
        .flag(
            Flag::new("load", FlagType::String)
                .description("load object file path")
                .alias("l"),
        )
        .flag(
            Flag::new("func", FlagType::String)
                .description("update function name")
                .alias("f"),
        );

    app.run(args);
}

fn usage() {
    println!("Usage: cli --pid [pid] --load [filepath] --func [function name]");
}

fn run(c: &Context) {
    let Ok(pid) = c.int_flag("pid") else { usage(); return; };
    let Ok(load_path) = c.string_flag("load") else { usage(); return; };
    let Ok(func) = c.string_flag("func") else { usage(); return; };

    if pid > 0 {
        let pid = pid as i32;
        let proc = Proc::new(pid).unwrap();
        load(proc);
        //            jmp(proc);
    } else {
        eprintln!("error: pid must positive number.");
    }
}
