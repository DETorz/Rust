fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) 
{
    let mut n = low; // first example...
    while n <= high
    {
        *total += n; // 1 + 2 + 3 + 4 + 5... + 100
        n += step;   // with step = 1, the result is like  `for i in 0..=high {*total += i}`
    }
}

fn main()
{
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
