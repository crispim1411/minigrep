use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //obtem argumentos passados
    let args: Vec<String> = env::args().collect();
    
    //configuração a partir dos argumentos
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

