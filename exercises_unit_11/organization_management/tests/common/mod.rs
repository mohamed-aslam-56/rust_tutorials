use organization_management::User;

pub fn setup_db()->Vec<User>{
    println!("Database is setup successfully!");
    Vec::new()

}

pub fn teardown_db(users:&mut Vec<User>){
    users.clear();
    println!("Database is torn down!");

}