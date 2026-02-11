use std::io;
fn countwords(input_sentence:&str){

    let mut words=0;
    let mut start=-1;

    let chars=input_sentence.chars();
    
    for l in chars{
        if l.is_ascii_alphabetic(){
            if start==-1{
                start+=1;
            }
        }
        else{
            if start==0 {
                words+=1;
            }
            start=-1;
        }
    }
    if start==0{
        words+=1;
    }

    println!("The number of words in {input_sentence} are : {words}");
}

fn main(){
    let n=5;

    for _  in 1..=n {
        let mut input_sentence=String::new();

        io::stdin()
        .read_line(&mut input_sentence)
        .expect("Could not read input sentence!");

        let input_sentence:String=input_sentence.trim().to_string();

        countwords(&input_sentence);
    }
    
}