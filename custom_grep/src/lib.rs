use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool
}

impl Config{

    pub fn build(input_config:&Vec<String>)->Result<Config,&'static str>{
        if input_config.len()<3{
            return Err("Not enough command line arguments!");
        }
        let query=&input_config[1];
        let file_path=&input_config[2];
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(Config{query:query.to_string(),file_path:file_path.to_string(),ignore_case})
    }

}

pub fn run(config_args:&Config)->Result<(),Box<dyn Error>>{
   let contents=fs::read_to_string(&config_args.file_path)?;
   let mut results=Vec::new();
   let query=&config_args.query;

   if config_args.ignore_case{
       results=search(&query,&contents);
   }
   else{
       results=search_case_insensitive(&query,&contents);
   }
   for line in results{
      println!("{line}");
   }
   Ok(())
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

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut result=Vec::new();
    let lower_query=query.to_lowercase();

    for line in contents.lines(){
        let lower_line=line.to_lowercase();
        if lower_line.contains(&lower_query){
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

    #[test]
    fn do_search_case_insensitive(){
        let query="OMe";
        let contents="You have to come now\nCan you come here\nCometh the man";
        assert_eq!(vec!["You have to come now","Can you come here","Cometh the man"],search_case_insensitive(&query,&contents));
    }

}
