use std::io;

fn append_string(input_string:String)->String{
    // input_string.push_str("doubled!");
    // input_string
    let k=String::from("doubled!");
    let result=format!("{}{}",input_string,k);
    result
}
fn main(){
    
    let mut input_string=String::new();
    io::stdin()
    .read_line(&mut input_string)
    .expect("Could not read the input string!");

    let input_string:String=input_string.trim().to_string();
    println!("Appended String,{}",append_string(input_string));
    //You can pass as immutable string inside a function which takes it as mutable,but vice versa is not possible

}