use std::collections::HashMap;
use std::io;

struct InventoryError{
    message:String
}

impl InventoryError{
    fn item_error()->Self{
        let message:String="Item does not exist".to_string();
        let item_error=InventoryError{message};
        item_error
    }

    fn out_of_stock()->Self{
        let message:String="Item is out of stock".to_string();
        let out_of_stock_error=InventoryError{message};
        out_of_stock_error
    }
}

fn main(){
    let mut inventory:HashMap<String,i32>=HashMap::new();
    
    let mut count:String=String::new();
    println!("Enter the number of items to be added:");

    io::stdin().
    read_line(&mut count)
    .expect("Could not read the input count!");
    
    let count:i32=count.trim().parse().expect("Invalid number!");

    for _ in 1..=count{

        let mut item_name=String::new();
        let mut item_count=String::new();

        println!("Enter the item name:");

        io::stdin().
        read_line(&mut item_name)
        .expect("Could not read the input count!");

        let item_name:String=item_name.trim().to_string();
        
        println!("Enter the item count:");

        io::stdin().
        read_line(&mut item_count)
        .expect("Could not read the input count!");

        let item_count:i32=item_count.trim().parse().expect("Could not read count");

        let count=inventory.entry(item_name).or_insert(0);
        *count+=item_count;
    }

    let mut purchase_count:String=String::new();
    println!("Enter the number of items to be purchases:");

    io::stdin().
    read_line(&mut purchase_count)
    .expect("Could not read the input count!");
    
    let purchase_count:i32=purchase_count.trim().parse().expect("Invalid number!");

    for _ in 1..=purchase_count{

        let mut item_name=String::new();
        let mut quantity=String::new();

        println!("Enter the item name:");

        io::stdin().
        read_line(&mut item_name)
        .expect("Could not read the input count!");

        let item_name:String=item_name.trim().to_string();
        
        println!("Enter the quantity purchased:");

        io::stdin().
        read_line(&mut quantity)
        .expect("Could not read the input count!");

        let quantity:i32=quantity.trim().parse().expect("Could not read count");

        match purchase_item(&item_name,quantity,&mut inventory){
            Err(e)=>println!("{}",e.message),
            Ok(_)=>println!("Purchase successful!"),
        }
    }

}

fn purchase_item(item:&str,quantity:i32,inventory:&mut HashMap<String,i32>)->Result<(),InventoryError>{
    let count=inventory.entry(item.to_string()).or_insert(0);
    if *count==0{
        let item_error=InventoryError::item_error();
        Err(item_error)
    }
    else if quantity>*count{
        let out_of_stock_error=InventoryError::out_of_stock();
        Err(out_of_stock_error)
    }
    else{
        *count-=quantity;
        Ok(())
    }
    
    
}
