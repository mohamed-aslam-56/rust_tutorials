use employee_management::office::roster::{self,Employee};

fn main(){
    let emp1=Employee{name:String::from("Henry"),id:32};
    let emp2=Employee{name:String::from("Harry"),id:20};
    let emp3=Employee{name:String::from("Larry"),id:16};
    
    let mut employees:Vec<Employee>=Vec::new();
    employees.push(emp1);
    employees.push(emp2); 
    employees.push(emp3);

    let search_string=String::from("Larry");
    roster::search(&employees,&search_string);

    

}