use organization_management::User;
mod common;

#[test]
pub fn test_user_can_save_user(){
    let mut db=common::setup_db();

    let valid_user=User{
        username:String::from("Rustacean"),
        password:String::from("secure123"),
    };

    let can_save=!(valid_user.password.len()<5)  
    && !(valid_user.username.contains("*"));
    assert!(can_save,"User should be saveable!");
    common::teardown_db(&mut db);
}

#[test]
fn test_cannot_save_invalid_username() {
    let invalid_user = User {
        username: String::from("Admin*"), // Contains '*'
        password: String::from("12345"),
    };

    let can_save = !invalid_user.username.contains("*");
    
    assert!(!can_save, "User with * in name should not be saved!");
}