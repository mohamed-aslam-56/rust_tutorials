use config_parser::{parse_setting};

fn main(){
    let s1="124923";
    let s2="-132132";
    let s3="12402-43d";

    println!("{:?}",parse_setting::<i32>(s1,"port"));
    println!("{:?}",parse_setting::<i32>(s2,"id"));
    println!("{:?}",parse_setting::<i32>(s3,"count"));


}