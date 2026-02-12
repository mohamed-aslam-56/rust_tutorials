enum Shape{
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32),
    Triangle(f32,f32,f32),
    CatchAll(f32)
}

impl Shape{
    fn calculate_area(&self)->f32{
        match self{
           Shape::Circle(r)=>std::f32::consts::PI*r*r,
           Shape::Square(s)=>s*s,
           Shape::Rectangle(w,h)=>w*h,
           Shape::Triangle(a,b,c)=>{
            
              let s=(a+b+c)/2.0;
              (s*(s-a)*(s-b)*(s-c)).sqrt()

           },
           _ =>-1.0//Catch all case

        }
    }
}

enum Medals{
    Gold,
    Silver,
    Bronze,
    Participation
}
enum Winners{
    Medal(Medals),
}

//Some methods
fn check(number:Option<i32>){
    assert_eq!(number.is_some(),true);
    println!("{}",number.is_some());

    let some_and=number.is_some_and(|x|x>10);
    assert_eq!(true,some_and);
    println!("{}",some_and);

    //assert_eq!(number.is_none(),true) This will panic when we pass Some(x) here
    println!("{}",number.is_none());

    let none_or=number.is_none_or(|x|x>10);
    assert_eq!(true,none_or);
    println!("{}",none_or);
}

fn point_details(medals:&Medals)->f32{
    match medals{
        Medals::Gold=>500.0,
        Medals::Silver=>250.0,
        Medals::Bronze=>125.0,
        _=>0.0 //Participation 
    }
}

//Calculate points based on winners
fn calculate_points(medals:&[Medals])->f32{
    let mut points=0.0;
    for medal in medals{
       points+=point_details(medal);
    }
    points   
}
fn main() {
    //Shapes num
    let t1=Shape::Triangle(0.5,2.5,0.3);
    let c1=Shape::Circle(2.34);
    let s1=Shape::Square(5.3);
    let r1=Shape::Rectangle(3.2,2.8);
    let g1=Shape::CatchAll(0.5);

    let x=Some(11);
    check(x);

    println!("{} {} {} {} {}",t1.calculate_area(),c1.calculate_area(),s1.calculate_area(),r1.calculate_area(),g1.calculate_area());

    let winners:[Medals;5]=[Medals::Gold,Medals::Gold,Medals::Silver,Medals::Bronze,Medals::Participation];
    let total=calculate_points(&winners);
    println!("{}",total);

    //Checking the first person is a gold medalist

    let gold_medalist=Winners::Medal(Medals::Gold);
    //Declaring these for pattern definition
    let val1=&winners[0];
    let val2=&winners[2];

    if let Winners::Medal(ref val1)=gold_medalist{
        println!("The first athlete is a gold medalist");
    };
    let Winners::Medal(ref val2)=gold_medalist else{
        println!("The third athlete is not a gold medalist");
        return;
    };



}
