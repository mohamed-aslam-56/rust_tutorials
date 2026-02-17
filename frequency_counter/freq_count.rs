use std::collections::HashMap;

fn main(){
   
    let input_sentence="Old is Gold Gold Golden Silver Silver Orange";
    match count_words(&input_sentence){
        Ok(map)=>println!("{:?}",map),
        Err(err)=>println!("{err}")
    }

}

fn count_words(input:&str)->Result<HashMap<String,i32>,String>{
    let mut map:HashMap<String,i32>=HashMap::new();

    if input.is_empty() {
        return Err("String is empty".to_string())
    }

    for word in input.split_whitespace(){
        *map.entry(word.to_string()).or_insert(0)+=1;
    }

    Ok(map)
}