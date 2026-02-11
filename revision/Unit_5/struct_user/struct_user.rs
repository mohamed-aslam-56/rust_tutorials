#[derive(Debug)]
struct User{
     username:String,
     email:String,
     department:String,
     designation:String
}

impl User{
    fn summary(&self){
        println!("UserName:{} \n Email:{} \n Department:{} \n Designation:{}",self.username,self.email,self.department,self.designation);
    }
}

fn main(){
    let user1=User{username:String::from("Mohamed Aslam"),email:String::from("mohamedaslam3283@gmail.com"),department:String::from("HR"),designation:String::from("SRE")};
    let user2=User{username:String::from("Lillian Garcia"),email:String::from("lilliangarcia283@gmail.com"),department:String::from("Finance"),designation:String::from("Accountant")};

    user1.summary();
    user2.summary();

    //If fields are &str we can just define strings in this form "" but to do the same using String type we use String::from()

}