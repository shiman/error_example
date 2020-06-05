use std::process;
use error_example;

fn try_main(args: i32) -> Result<(), error_example::Error> {
    let res = error_example::detect(args)?;
    println!("{}", res);
    Ok(())
}

fn main() {
    let args: i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    if let Err(err) = try_main(args) {
        eprintln!("{}", err);
        process::exit(2);
    }
    println!("Hello, world!");
}
