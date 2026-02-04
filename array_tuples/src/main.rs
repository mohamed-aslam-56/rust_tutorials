fn main() {
    //Tuple

    let x:(u8,i32,char,f64)=(9,-45,'h',9.432);

    let (a,b,c,d)=x;
    println!(" {a} {b} {c} {d}");

    //Arrays
    let array:[f32;10]=[0.5,0.8,0.9,0.9,0.3,2.342,-243.422,0.5,0.2,1.7];
    
    for number in array{
        println!("{}",number);
    }

    for i in (0..array.len()).rev(){
        println!("{}",array[i]);
    }

    //not possible in tuple
    for number in array.iter(){
        println!("{number}");
    }


}
