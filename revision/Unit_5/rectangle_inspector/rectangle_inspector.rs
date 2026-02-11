#[derive(Debug)]
struct Rectangle{
    height:f32,
    width:f32
}

impl Rectangle{

    fn square(height:f32)->Self{
        Self{
            height,
            width:height
        }
    }

    fn is_square(&self)->bool{
        self.height==self.width
    }

    fn canfit(&self,other:&Rectangle)->bool{
        self.height>=other.height && self.width>=other.width
    }
}

fn createRectangle(height:f32,width:f32)->Rectangle{
    Rectangle{
        height,
        width
    }
}

fn main(){
    let h1=4.3;let w1=2.5;
    let h2=8.3;let w2=1.2;

    let r1=createRectangle(h1,w1);
    let r2=createRectangle(h2,w2);
    let r3=Rectangle::square(h1);
    let r4=Rectangle::square(h2);

    if r1.is_square(){
        println!("{:?} is square",r1);
    }
    else{
        println!("{:?} is not square",r1);
    }

    if r2.is_square(){
        println!("{:?} is square",r2);
    }
    else{
        println!("{:?} is not square",r2);
    }

    if r3.is_square(){
        println!("{:?} is square",r3);
    }
    else{
        println!("{:?} is not square",r3);
    }

    if r4.is_square(){
        println!("{:?} is square",r4);
    }
    else{
        println!("{:?} is not square",r4);
    }

    if r1.canfit(&r2){
        println!("{:?} can fit in {:?}",r1,r2);
    }
    else{
        println!("{:?} cannot fit in {:?}",r1,r2);
    }

    if r2.canfit(&r1){
        println!("{:?} can fit in {:?}",r2,r1);
    }
    else{
        println!("{:?} cannot fit in {:?}",r2,r1);
    }
}