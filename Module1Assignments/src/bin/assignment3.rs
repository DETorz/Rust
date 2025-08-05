use std::io;

fn check_guess(guess: i32, secret: i32) -> i32
{
    if secret < guess
    {
        return 1;
    }
    else if secret > guess
    {
        return -1;
    }
    else
    {
        return 0;
    }
}

fn main()
{
    let secret = 10;
    let mut tries = 1;
    println!("Welcome to the number guessing game! Start by guessing an integer 1 - 100: ");
    loop                      // shorter than while true
    {
           let mut guess = String::new(); // intiates a buffer for some size
            let _ = io::stdin()          // halts the program for a standard input
            .read_line(&mut guess);     // fill the next line with a string
            let guess: i32 = guess.trim().parse().unwrap(); // shadowing the old variable name and parsing it from string to an int
        if check_guess(guess, secret) == -1
        {
            println!("Your guess was too low! Try again.");
            tries += 1;
        }

        else if check_guess(guess, secret) == 1
        {
            println!("Your guess was too high! Try again.");
            tries += 1;
        }

        else if check_guess(guess, secret) == 0
        {
            if tries == 1
            {
                println!("You got the answer in 1 try!");
                break;
            }
            else
            {
                println!("You got the answer in {} tries!", tries);
                break;
            }
        }
    }
}
