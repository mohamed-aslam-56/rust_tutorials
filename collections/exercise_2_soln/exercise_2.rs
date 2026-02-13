use std::io;

fn isVowel(l:char)->bool{
    l=='a'||l=='e'||l=='i'||l=='o'||l=='u'
    ||l=='A'||l=='E'||l=='I'||l=='O'||l=='U'
}

fn main(){
    let n=3;
    
    for _ in  1..=n{
        let mut string=String::new();

        io::stdin()
        .read_line(&mut string)
        .expect("Could not read the string");

        let word=string.trim();
        let first_char=word.chars().next().unwrap();

        if isVowel(first_char){
            println!("{}-hay",word);
        }
        else{
            let rest=&word[1..];
            println!("{}-{}ay",rest,first_char)
        }
    
    }
}