use std::io;

fn main() {
    let mut f_val=String::new();
    io::stdin()
    .read_line(&mut f_val)
    .expect("Invalid Input");
    let f_val:f32=f_val.trim().parse().expect("Invalid fahrenheit entry!");

    let mut c_val=String::new();
    io::stdin()
    .read_line(&mut c_val)
    .expect("Invalid Input");
    let c_val:f32=c_val.trim().parse().expect("Invalid celsius entry!");

    let mut n_val=String::new();
    io::stdin()
    .read_line(&mut n_val)
    .expect("Invalid Input");
    let n_val:i32=n_val.trim().parse().expect("Invalid nth term!");

    println!("{}",f_to_c(f_val));
    println!("{}",c_to_f(c_val));
    println!("{}",nth_fibonacci(n_val))
    
}

//Fahrenheit to Celsius
fn f_to_c(f_val:f32)->f32{
    let celsius:f32=(f_val-32.0)*5.0/9.0;
    celsius //Return celsius
}

//Celsius to Fahrenheit
fn c_to_f(c_val:f32)->f32{
    let fahrenheit:f32=(c_val*9.0)/5.0;
    fahrenheit //Return fahrenheit
}

//Nth fibonacci number
fn nth_fibonacci(n:i32)->i32{
    if n==1{
        0 //Return 0
    }
    else if n==1||n==2{
        1 //Return 1
    }
    else{
        let mut first:i32=0;
        let mut second:i32=1;

        for _  in 3..=n+1 {
            let sum=first+second;
            first=second;
            second=sum;
        }
        second //Return second
    }
}

