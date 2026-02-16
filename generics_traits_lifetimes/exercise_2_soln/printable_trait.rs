pub trait Printable{
    fn format_pretty(&self)->String;
}
struct NewsArticle{
    headline:String,
    author:String,
    topic:String,
    date:String
}

impl Printable for NewsArticle{
    fn format_pretty(&self)->String{
        format!("Headline: {}\nAuthor:{}",self.headline,self.author)
    }
}

impl Printable for i32{
    fn format_pretty(&self)->String{
        format!("The number printed is {}!",self)
    }
}

fn print_it<T:Printable>(item:&T){
    let res=item.format_pretty();
    println!("{}",res);
}

fn main(){
    let article=NewsArticle{headline:String::from("India win against Pakistan by 70 runs"),
                         author:String::from("JK rowling"),
                         topic:String::from("Sports analysis"),
                         date:String::from("16-02-2026")
                        };
    let x:i32=995;
    print_it(&article);
    print_it(&x);
}