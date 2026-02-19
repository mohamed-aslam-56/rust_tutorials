pub fn reverse_string(string:&str)->String{
    let mut reversed_chars:Vec<char>=Vec::new();

    for l in string.chars(){
        reversed_chars.insert(0,l);
    }

    let result:String=reversed_chars.into_iter().collect();
    result
}
