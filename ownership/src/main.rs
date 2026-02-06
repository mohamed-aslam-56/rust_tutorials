fn main() {

    //This gives error as s1 is moved to s2,shallow copy
    //Pointer,capacity,length of s1 is copied and s2 uses it
    //let s1=String::from("Vanga vanakam anna");
    //let s2=s1;

    //deep copy,stack and heap allocation
    let s1=String::from("Vanga vanakam anna");
    let s2=s1.clone();

    println!("{s1}");
    mutable_references();
    mutable_immutable_references();
}

fn mutable_references(){
    let mut s=String::from("hello");

    let r1=&mut s;
    //let r2=&mut s;//Only one  mutable reference for a value

    println!("{r1}");//s1 goes out of scope

    let r2=&mut s;//Only one  mutable reference for a value
    println!("{r2}");
}

fn mutable_immutable_references(){

//     let mut s=String::from("hello");
//     let s1=&s;
//     let s2=&s;
//     let s3=&mut s;
//     println!("{s1} {s2} {s3}");//s is borrowed as mutable in the last,so it can't be immutable

        // let mut s=String::from("hello");
        // let s1=&mut s;
        // let s2=&s;
        // let s3=&s;
        // println!("{s1} {s2} {s3}");//cannot borrow `s` as immutable because it is also borrowed as mutable
          
}
