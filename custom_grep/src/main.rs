use std::env;
use custom_grep::{Config,run};
use std::process;

fn main(){
    let input_config:Vec<String>=env::args().collect();
    let config_args:Config=Config::build(&input_config).unwrap_or_else(|err|{
        eprintln!("Problem parsing command line arguments:{err}");
        process::exit(1);
    });
    
    if let Err(e)=run(config_args){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}