use coloring::parser;
use coloring::scheduler;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Must provide one argument to the path of a .color file");
        std::process::exit(0)
    }
    let arg = args[1].clone();
    if !arg.ends_with(".color") {
        println!("Must provide one argument to the path of a .color file");
        std::process::exit(0)
    }
    let input_string = std::fs::read_to_string(arg).expect("couldn't read input");
    let program = parser::parse(&input_string).expect("Parsing failed");
    let result = scheduler::schedule(program);
    println!("{}", result);
}
