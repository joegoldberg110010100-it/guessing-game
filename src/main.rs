use std::io;
use rand::Rng;

fn main() {
    let mut random: u8 = rand::thread_rng().gen_range(1..=20);
    let mut trial: u32 = 5;
    let mut win: bool = false;
    //==========================================================
    println!("Hi! Guess the number! ");
    loop {
        if trial > 0 {
            win = guess(random, &mut trial);
            if win == true{
                println!("U win this game!");
                break;
            }else {
                println!("Oh, u lose, try again!");
            }
        }else {
            println!("U haven't trial more, sory.");
            break;
        }
    }

}

fn guess(rand: u8, attemp:&mut u32) -> bool{
    //======make=number===============================================
    println!("Write ur number (1-20) here! u have {attemp} attemp!");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Error os reads lol");
    let answer: u8 = answer.trim().parse().expect("Do u sure write not number?)");
    //======check======================================================
    if answer == rand {
        println!("U WIN!!!");
        true
    }else if answer > rand {
        println!("Too big!");
        *attemp -= 1;
        false
    }else {
        println!("Too small!");
        *attemp -= 1;
        false
    }
}
