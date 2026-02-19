use ageparser::parse_age;
#[test]
fn do_parse_age()->Result<(),String>{
    let age="94";
    let _res=parse_age(age)?;
    Ok(())
}

#[test]
fn do_invalid_age(){
    let age="124";
    let res=parse_age(age);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(),"Invalid age range.");
}