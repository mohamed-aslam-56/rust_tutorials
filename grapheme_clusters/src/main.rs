use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s="ááàǹúç";

    for l in s.graphemes(true){
        println!("{}",l);
    }
    let graphemes_vec:Vec<&str>=s.graphemes(true).collect();
    let bytes_vec:Vec<char>=s.chars().collect();
    let len:i8=bytes_vec.len() as i8;
    println!("{:?}",graphemes_vec);
    println!("{:?}",bytes_vec);
    println!("{:?}",len);

    let complex_string="rust 🦀 é";
    let complex_graphemes:Vec<&str>=complex_string.graphemes(true).collect();
    println!("{:?}",complex_graphemes);
}
