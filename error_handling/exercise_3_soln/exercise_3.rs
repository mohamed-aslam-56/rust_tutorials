use std::fs;
use std::error::Error;
use std::io::ErrorKind;

fn main()->Result<(),Box<dyn Error>>{
    let username=read_username_from_file();
    println!("Username {:?}",username);
    Ok(())
}
fn read_username_from_file()->Result<String,Box<dyn Error>>{
    let path="config.txt";

    match fs::read_to_string(path){
        Ok(content)=>Ok(content),
        Err(e)if e.kind()==ErrorKind::NotFound=>{
            let guest="guest".to_string();
            fs::write(path,&guest);
            Ok(guest)
        }
        Err(e)=>Err(e.into()),
        
    }
}
//Traits must be behind a pointer,to do it use Box to define 
// a size during compile time