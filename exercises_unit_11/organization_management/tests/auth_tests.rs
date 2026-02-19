use organization_management::User;
mod common;

#[test]
pub fn verify_user_test(){
    let mut db=common::setup_db();
    let test_user=User{
        username:String::from("Alice"),
        password:String::from("12345")
    };
    
    db.push(test_user);
    assert_eq!(db.len(),1);
    common::teardown_db(&mut db);
}                    