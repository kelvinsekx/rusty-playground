fn main() {
    let stdin = std::io::stdin();
    let mut convert_type = String::new();

    println!("What do you want me to do?");

    loop {
        println!("should I convert from Fahrenheit to Celcius\n or Celcius to Fahrenheit ");
        println!("If from F to C, type 'C'\n If from C to F, type 'F'");

        stdin
            .read_line(&mut convert_type)
            .expect("Problem fetching convert type");

        let convert_type = convert_type.chars().nth(0).unwrap().to_ascii_uppercase();

        if convert_type == 'F' {
            convert(convert_to_fahrenheit);
        }
        if convert_type == 'C' {
            convert(convert_to_celsius);
        } else {
            println!("This is not a valid conversion type")
        }
        println!("****************CLICK CTR+C TO EXIT********************")
    }
}

fn convert(cn: fn(i32) -> ()) {
    let stdin = std::io::stdin();
    let mut text = String::new();

    println!("input a temperature I should convert");

    stdin
        .read_line(&mut text)
        .expect("Problem receiving integer to convert");

    println!("Your input is received.");

    let text: i32 = match text.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("something wrong parsing text"),
    };
    cn(text);
}

fn convert_to_fahrenheit(value: i32) -> () {
    let result: f32 = (value as f32 * 1.8) + 32 as f32;
    println!("{value} celsius = '{result} Fahrenheit'")
}

fn convert_to_celsius(value: i32) -> () {
    let result: f32 = (value - 32) as f32 / 1.8;
    println!("{value} fahrenheit = '{result} Celsius'")
}
