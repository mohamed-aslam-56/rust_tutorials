pub trait Summary{
    fn summarize(&self)->String;
    fn compare_summaries<U:Summary>(&self,other:&U)->String{
        if self.summarize().len()>other.summarize().len(){
            self.summarize()
        }
        else{
            other.summarize()
        }
    }
}

struct NewsSummary{
    summary:String,
}

impl Summary for NewsSummary{
    fn summarize(&self)->String{
        self.summary.clone()
    }
}


fn main(){
    let s1:NewsSummary=NewsSummary{summary:String::from("India win the world cup after 28 years")};
    let s2:NewsSummary=NewsSummary{summary:String::from("Travis Head traumatizes India")};
    let res=s1.compare_summaries(&s2);
    println!("{:?}",res);
}
