use std::env;
use custom_grep::{Config,run,search};
use std::process;

fn main(){
    let input_config:Vec<String>=env::args().collect();
    let config_args:Config=Config::build(&input_config).unwrap_or_else(|err|{
        eprintln!("Problem parsing command line arguments:{err}");
        process::exit(1);
    });

    let contents=run(&config_args);
    
    if let Err(e)=contents{
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
    
    let contents=contents.unwrap();

    let result=search(&config_args.query,&contents);
    println!("{:?}",result);
}