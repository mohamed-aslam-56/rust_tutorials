use std::fmt::Display;

struct MyWrappedList<T>(Vec<T>);

impl Default for MyWrappedList<i32>{
    fn default()->Self{
        MyWrappedList(vec![42])
    }
}

impl<T:Display>MyWrappedList<T>{
    fn printvec(&self){
        for item in &self.0{
            print!("{} | ",item);
        }
        println!();
    }
}

fn main(){
    let vector=MyWrappedList::default();
    vector.printvec();
}