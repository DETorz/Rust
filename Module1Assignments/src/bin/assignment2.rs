const ARRAY: [i32; 10] = [3, 4, 5, 6, 7, 44, 8, 15, 11, 120]; // [a, b] after the colon... a is the datatype of the items, and b is number of items

fn is_even(n: i32) -> bool
{
    return n % 2 == 0;
}

fn main()
{
    println!("Even number checker: ");
    for i in &ARRAY
    {
        if is_even(*i) == true
        {
            println!("{} is even", i);
        }
        else
        {
            println!("{} is odd", i);
        }
    }

    println!("\nFizzBuzz: ");
    for i in &ARRAY
    {
        match(*i % 3, *i % 5)
        {
            (0, 0) => print!("FizzBuzz, "),
            (0, _) => print!("Fizz, "),
            (_, 0) => print!("Buzz, "),
                _  => print!("{}, ", *i),  // in rust, a simple underscore means "neither" or a variable to throw away
        }
    }
    println!();
    println!("\nLargest number in an array and the sum: ");
    let mut sum = 0;
    let mut max = 0;
    let mut i = 0;
    while i < ARRAY.len()
    {
        if ARRAY[i] > max
        {
            max = ARRAY[i];
        }
        sum += ARRAY[i];
        i += 1;
    }

    println!("The max number found was: {}", max);
    println!("The sum of all the numbers was: {}\n", sum);
}