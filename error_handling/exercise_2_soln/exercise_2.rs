use std::io;
fn main(){
   let mut age:String=String::new();
   println!("Enter the age of the person!");

   io::stdin()
   .read_line(&mut age)
   .expect("Could not read the input string");

   match parse_age(&age){
       Ok(age)=>println!("The age of person is {age}"),
       Err(msg)=>println!("{msg}"),
   }

}

fn parse_age(age:&String)->Result<u32,String>{
    let parsed_age:u32=age.trim().parse().expect("Invalid age!");
    if parsed_age<1||parsed_age>120{
        Err("Age is out of the realistic range".to_string())
    }
    else{
        Ok(parsed_age)
    }
}