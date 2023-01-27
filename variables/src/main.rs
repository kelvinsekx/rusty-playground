fn main() {
    let stdin = std::io::stdin();
    println!("What do you want me to do?");

    loop {
        let mut convert_type = String::new();
        println!("should I convert from Fahrenheit to Celcius\n or Celcius to Fahrenheit ");
        println!("If from F to C, type 'C'\n If from C to F, type 'F'");

        println!("{}", convert_type);

        stdin
            .read_line(&mut convert_type)
            .expect("Problem fetching convert type");

        let convert_type = convert_type.chars().nth(0).unwrap().to_ascii_uppercase();

        let arr: [(char, fn(i32)); 2] = [('F', convert_to_fahrenheit), ('C', convert_to_celsius)];

        for item in arr.iter() {
            if convert_type == item.0 {
                convert(item.1)
            }
        }
        if convert_type != arr[0].0 && convert_type != arr[1].0 {
            println!("This is not a valid conversion type");
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
    let result = (value as f32 * 1.8) + 32 as f32;
    println!("{value} celsius = '{result} Fahrenheit'")
}

fn convert_to_celsius(value: i32) -> () {
    let result = (value - 32) as f32 / 1.8;
    println!("{value} fahrenheit = '{result} Celsius'")
}
