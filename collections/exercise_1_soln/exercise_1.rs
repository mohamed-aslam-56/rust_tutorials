use std::collections::HashMap;

fn median(list:&mut Vec<i32>)->f32{
    let n=list.len();
    list.sort();
    

    if n%2==1{
       list[n/2]as f32
    }
    else{
        (list[n/2]+list[n/2 -1])as f32/2.0
    }
}

fn mode(list:&Vec<i32>)->i32{
    let mut map=HashMap::new();

    for num in list{
        let count=map.entry(*num).or_insert(0);
        *count+=1;
    }

    let mut max=0;
    let mut key=0;

    for(k,v)in &map{
        if *v>max{
            key=*k;
            max=*v;
        }
    }

    key

}

fn main(){

    let mut vec1:Vec<i32> =Vec::new();
    vec1.push(5);vec1.push(10);
    vec1.push(3);vec1.push(12);
    vec1.push(12);vec1.push(5);
    vec1.push(18);

    println!("Median: {}",median(&mut vec1));
    println!("Mode: {}",mode(&vec1));
}