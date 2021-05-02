use minigrep::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred: {}", err);
        std::process::exit(1);
    });

    run(config).unwrap_or_else(|err| eprintln!("Error occurred: {}", err));
}
