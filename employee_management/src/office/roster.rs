pub struct Employee{
    pub name:String,
    pub id:u32
}

pub fn search(employees:&Vec<Employee>,name:&String){
    let mut present=false;
    for i in 0..employees.len(){
        let employee=&employees[i];
        if employee.name==*name {
            println!("Employee is found with id:{}",{employee.id});
            present=true;
            break;
        }
    }

    if !present{
        println!("Employee is not found")
    }
}