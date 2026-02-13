use std::io;
use std::collections::HashMap;
fn main(){
    let mut dept_employees:HashMap<String,Vec<String>>=HashMap::new();

    let mut n=String::new();

    println!("Enter the number of people to add:");

    io::stdin()
    .read_line(&mut n)
    .expect("Could not read the input");

    let n=n.trim().parse().expect("Invalid input entry");

    for _ in 1..=n{

        let mut query:String=String::new();
        
        io::stdin()
        .read_line(&mut query)
        .expect("Could not read the name of employee!");

        let query:String=query.trim().to_string();

        //let words:Vec<&str>=query.split_whitespace().collect();
        let words:Vec<String>=query.split_whitespace().map(String::from).collect();
        let mut flag=0;

        for i in 0..words.len(){
            if words[i]=="to"{
                flag=i;
                break;
            }
        }

        let name=words[1..flag].join(" ");
        let dept=words[flag+1..].join(" ");

        add_employee(name,dept,& mut dept_employees);
    }

    get_employees(&mut dept_employees);

}

fn add_employee(name:String,dept:String,dept_employees:&mut HashMap<String,Vec<String>>){
    let list=dept_employees.entry(dept).or_insert(Vec::new());
    list.push(name);
}

fn get_employees(dept_employees:&mut HashMap<String,Vec<String>>){

    for (name,employees) in dept_employees{
        employees.sort();
        println!("Department Name:{}",name);
        for employee in employees{
            println!("Name:{}",employee);
        }
    }
}