use std::io;

fn main() {
   for _ in 1..=3{
        let mut words=String::new();

        io::stdin()
        .read_line(& mut words)
        .expect("Invalid ASCII characters");

        let words:String=words.trim().to_string();
        let second_word:&str=words.trim();

        find_the_first_word(&words);
        find_the_first_word(&second_word);
   }

   array_comp();
   
}

fn find_the_first_word(words:&str){
    let bytes=words.as_bytes();//We convert it to bytes as they are multi-character represented as a single byte
    let mut j=words.len();

    for (i,&item)in bytes.iter().enumerate(){
        if item==b' '{
            j=i;
            break;
        }
    }


   
    println!("The first word in the sentence is: {}",&words[..j]);

}

fn array_comp(){
    let a=[3,9,8,12,15];
    let slice=&a[1..3];
    assert_eq!(slice,&[2,3]);
}