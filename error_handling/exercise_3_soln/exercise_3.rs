use std::fs::{self,File};
use std::error::Error;
use std::io::ErrorKind;

fn main()->Result<(),Box<dyn Error>>{
    let username=read_username_from_file();
    println!("Username {:?}",username);
    Ok(())
}
fn read_username_from_file()->Result<String,Box<dyn Error>>{
    let path="config.txt";
    let file=File::open(&path);

    match file{
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>{
                let mut newFile=File::create(&path)?;
                let word="guest".to_string();
                fs::write(path,&word)?;
                Ok(word)
            }
            _=>Err("File cannot be opened".into())
        }

        Ok(fc)=>{
            let content=fs::read_to_string(path)?;
            Ok(content)
            
        }
        
    }
    
    

}
//Traits must be behind a pointer,to do it use Box to define 
// a size during compile time