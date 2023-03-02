use prpara::{core, types::Pid};
use seahorse::{App, Context, Flag, FlagType};
use std::{env, path::Path};

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

    // separate function names
    let str_vec: Vec<&str> = func.split(":").collect();
    let old_func = str_vec[0].to_string();
    let new_func = str_vec[1].to_string();

    let object_path = Path::new(&load_path);

    if pid > 0 {
        let pid = pid as Pid;
        let mut target = core::new(pid).unwrap();

        // parasite
        target
            .parasite_func(object_path, old_func, new_func)
            .unwrap();
    } else {
        eprintln!("error: pid must positive number.");
    }
}
