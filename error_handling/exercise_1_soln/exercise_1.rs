fn main(){
    let vector:Vec<i32>=vec![1,3,4,5,2];
    match get_first_element(&vector){
        Ok(value)=>println!("First element: {value}"),
        Err(msg)=>println!("{msg}"),
    }
}
fn get_first_elemerror_handling/exercise_1_soln/exercise_1 error_handling/exercise_1_soln/exercise_1.rsent(vector:&[i32])->Result<i32,String>{
    if vector.is_empty(){
        Err("Cannot get element from empty list".to_string())
    }
    else{
        Ok(vector[0])
    }
}