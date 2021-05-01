use std::io;

fn main() {
    loop {
        println!("Enter a temperature in fahrenheit");
        let mut temp = String::new();
        // read the temperature from std in
        io::stdin()
            .read_line(&mut temp)
            .expect("This should never happen");
        // redeclare temp as the parsed version
        let temp: f64 = match temp.trim().parse::<f64>() {
            Ok(number) => number,
            // if it's not a number, restart the loop
            Err(_) => continue,
        };
        // temp is always a number here, so we can just call the conversion
        let converted_temp = convert_f_to_c(temp);
        println!("The converted temperature is {}C", converted_temp);
    }
}

fn convert_f_to_c(f: f64) -> f64 {
    return (f - 32.0) * (5.0 / 9.0);
}
