use std::io;

fn main(){
    let mut sticks:i8=20;
    let mut is_player_one:bool=false;

    while sticks > 1{
        let mut stick_count=String::new();

        println!("Enter number of sticks to pick(1-3):");

        io::stdin()
        .read_line(&mut stick_count)
        .expect("Could not read input");

        let stick_count:i8=stick_count.trim().parse().expect("Invalid number");
        sticks-=stick_count;
        is_player_one=!is_player_one;   
    }


    if sticks<=0{
        if is_player_one{
            println!("Player 1 loses");
        }
        else{
            println!("Player 2 loses");
        }
    }
    else{
        if is_player_one{
            println!("Player 2 loses");
        }
        else{
            println!("Player 1 loses");
        }
    }
}