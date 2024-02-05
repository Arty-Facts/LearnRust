use rand::Rng;
fn main() {
    println!("Guess the number!");

    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        print!("You guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
