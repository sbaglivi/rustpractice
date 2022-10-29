use rand::Rng;
use std::cmp::Ordering;

fn get_and_convert_temperature(){
    let temperature = get_temperature();
    let current_scale = get_scale();
    let converted_temperature: f32 = convert_temperature(temperature, current_scale);
    println!("{} in celsius is {}!", temperature, converted_temperature);
}

fn get_scale() -> char {
    let mut input = String::new();
    loop {
        println!("Enter the current scale, 'c' for Celsius 'f' for Fahreneit");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let current_scale: char = match input.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };
        if current_scale == 'c' || current_scale == 'f' {
            break current_scale;
        };
    }
}

fn get_temperature() -> f32 {
    let mut input = String::new();
    loop {
        println!("Enter the temperature to convert");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let temperature: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break temperature;
    }
}

// Convert temperatures between Fahrenheit and Celsius.
fn convert_temperature(temperature: f32, current_scale: char) -> f32 {
    match current_scale {
        'f' => (temperature - 32.0) * 5.0 / 9.0,
        'c' => (temperature * 9.0 / 5.0) + 32.0,
        _ => {
            println!("Current scale ({current_scale}) is not valid, expected 'f' or 'c'");
            temperature
        },
    }
}

fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();
        println!("Please enter your guess");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}, secret number was {}", input, secret_number);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => println!("Too equal"),
        };
    }
}
fn get_positive_number() -> i32 {
    let mut input = String::new();
    loop {
        println!("Enter a number");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if number >= 0 {
            break number;
        }
    }
}

// Generate the nth Fibonacci number.
fn get_fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1|2 => 1,
        _ => get_fibonacci(n-1) + get_fibonacci(n-2),
    }
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn print_song() {
    const ORDINAL_NUMBERS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelveth"];
    const SECOND_LINE: &str = "My good friends brought me";
    const FRIEND_GIFTS: [&str; 12] = ["And a song for the Christmas tree", "Two candy canes", "Three boughs of holly", "Four colored lights", "A shining star", "Little silver bells", "Candles a-glowing", "Gold and silver tinsel", "A guardian angel", "Some mistletoe", "Gifts for one and all", "All their good wishes"];
    for i in 0..12 {
        print_first_song_line(ORDINAL_NUMBERS[i]);
        println!("{SECOND_LINE}");
        if i == 0 {
            println!("A song and a Christmas tree");
        } else {
            for j in 0..i+1 {
                println!("{}",FRIEND_GIFTS[i-j]);
            }
        }
        println!();
    }
}

fn print_first_song_line(ordinal_day: &str) {
    println!("On the {ordinal_day} day of Christmas");
}