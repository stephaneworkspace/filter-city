use filter_city::filter_city;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguements");
        process::exit(0);
    } else if args.len() > 2 {
        eprintln!("to many arguments");
        process::exit(0);
    }
    let name: String = args[1].clone();

    let search: Vec<filter_city::City> = filter_city(name);
    for s in search {
        println!("{:?}", s);
    }
}
