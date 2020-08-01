use filter_city::filter_city;
use std::env;
use std::process;

struct Arguments {
    city_name: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguements");
        } else if args.len() > 2 {
            return Err("to many arguements");
        }
        let f = args[1].clone();
        Ok(Arguments {
            city_name: args[1].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in &args {
        println!("{}", i);
    }
    println!("{:?}", args);

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing argument: {}", err);
        process::exit(0);
    });

    let search: Vec<filter_city::City> =
        filter_city(arguments.city_name.as_str());
    for s in search {
        println!("{:?}", s);
    }
}
