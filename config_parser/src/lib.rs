use std::str::FromStr;

#[derive(Debug)]
pub enum ConfigError{
    MissingKey(String),
    InvalidFormat(String),
    EmptySource(String)
}

pub fn parse_setting<T:FromStr>(data:&str,key:&str)->Result<T,ConfigError>{
    if key.is_empty(){
        return Err(ConfigError::MissingKey("Key is empty.Please provide a key!".to_string()));
    }
    if data.is_empty(){
        return Err(ConfigError::EmptySource("Source data is missing".to_string()));
    }
    //map_err converts standard error to custom error format
    data.trim().parse::<T>().map_err(|_| {
        ConfigError::InvalidFormat(format!("Value for '{}' is not the correct type",key))
    })
    
}
