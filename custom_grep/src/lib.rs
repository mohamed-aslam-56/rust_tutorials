use std::fs;
use std::error::Error;

pub struct Config{
    pub query:String,
    pub file_path:String
}

impl Config{

    pub fn build(input_config:&Vec<String>)->Result<Config,&'static str>{
        if input_config.len()<3{
            return Err("Not enough command line arguments!");
        }
        let query=&input_config[1];
        let file_path=&input_config[2];
        Ok(Config{query:query.to_string(),file_path:file_path.to_string()})
    }

}

pub fn run(config_args:&Config)->Result<String,Box<dyn Error>>{
   let contents=fs::read_to_string(&config_args.file_path)?;
   println!("Contents: {contents}");
   Ok(contents)
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut result=Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]

    fn do_search(){
        let query="Big dawgs";
        let contents="Hi Big dawgs\nO yeah yeah Big dawgs\nLalalalala Big dawgs";
        assert_eq!(vec!["Hi Big dawgs","O yeah yeah Big dawgs","Lalalalala Big dawgs"],search(&query,&contents));

    }
}
