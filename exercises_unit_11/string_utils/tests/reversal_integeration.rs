use string_utils::reverse_string;

mod common;

#[test]
fn it_reverse_string(){
    let string=common::get_test_string();
    let actual=reverse_string(&string);
    let expected:String=string.chars().rev().collect();
    assert_eq!(actual,expected);
    
}