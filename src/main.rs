use std::env;
use std::process;

use my_compressor::Config;
pub use my_compressor::huffman;


fn main() {
    // -----  Config -----

    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // ----- Run Logic and check errors -----
    if let Err(e) = my_compressor::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }



    process::exit(0);
}
