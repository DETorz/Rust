const FREEZING_WATER_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64
{
    return 5.0 *(f - 32.0) / 9.0;
}

fn celsius_to_fahrenheit(c: f64) -> f64
{
   return 9.0 *(c / 5.0) + 32.0;
}

fn main()
{
    let mut temperature: f64 = FREEZING_WATER_F;
    println!("Starting temperature: {}°F\n", temperature);
    for i in 1..=5
    {
        temperature = fahrenheit_to_celsius(temperature);
        println!("Iteration {}: {}°C", i, temperature);
        temperature = celsius_to_fahrenheit(temperature);
         println!("or {}°F\n------------------------------\n",temperature);
         temperature += 1.0;
    }
}