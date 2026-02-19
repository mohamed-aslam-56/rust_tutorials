pub fn parse_age(age:&str)->Result<u32,String>{
    let parsed_age=match age.trim().parse::<u32>(){
        Ok(num)=>num,
        Err(_)=>return Err("Invalid number".to_string())
    };

    if parsed_age<1||parsed_age>120{
        return Err("Invalid age range.".to_string());
    }
    Ok(parsed_age)
}