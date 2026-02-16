#[derive(Debug)]
struct Pair<T>{
    name:T,
    dept:T
}

impl<T> Pair<T>{
    fn new(name:T,dept:T)->Self{
        Self{
            name,
            dept
        }
    }

    fn swap(name:T,dept:T)->Self{
        Self{
            name:dept,
            dept:name
        }
    }
}

fn main(){
    let name:String=String::from("John");
    let dept:String=String::from("Finance");

    let p1=Pair::new(&name,&dept);
    let p2=Pair::swap(&name,&dept);

    println!("Pair:{:?}",p1);
    println!("Pair:{:?}",p2);

}