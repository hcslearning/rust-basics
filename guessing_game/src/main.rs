use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    loop {
        println!("#######################################");
        println!("Guess the number!");
        println!("Please input your guess:");
        
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("El número secreto es {secret_number}");
    
        // let -> create a constant 
        // let mut -> create a variable (mutable)
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line")
        ;
        let guess:u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Debe ingresar un número, no se permiten otro tipo de caracteres.");
                continue;
            }
        };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),            
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => { 
                println!("You win!!");
                break; // exit from loop 
            },
        }
    }
}
