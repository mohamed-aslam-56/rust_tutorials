#[derive(Debug)]
struct Triangle{
    base:f32,
    height:f32
}

impl Triangle{
    fn area(&self)->f32{
        self.base*self.height*0.5
    }

    fn isValid(&self)->bool{
        self.height>0.0 && self.base>0.0
    }

    fn equilateral_triangle(size:f32)->Self{
        Self{
            base:size,
            height:size
        }
    }
}

//Tuple structs
#[derive(Debug)]
struct Student(String,i32,char,f32);

fn main() {
    let t1=createTriangle(4.3,5.2);
    let s1=Student(String::from("Aslam"),2993,'A',93.5);
    println!("{t1:?}");
    println!("{s1:?}");
    println!("{}",t1.area());
    println!("{}",t1.isValid());
    let t2=Triangle::equilateral_triangle(4.0);
    println!("{t2:?}");
}

fn createTriangle(base:f32,height:f32)->Triangle{
    Triangle{
        //Fieldinit shorthand assignment
        base,
        height
    }
}
