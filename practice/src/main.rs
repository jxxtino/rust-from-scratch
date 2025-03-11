fn main(){
    fibonacci();
    temperature_converter();
}

fn temperature_converter(){
    let celsius_temp: i32 = 34;
    let fahr_temp: i32 = 67;

    println!("Fahrenheit Temperature: {fahr_temp}");
    println!("Celsius Temperature: {celsius_temp}");

    let f_to_c: i32 = (fahr_temp - 32) * 5/9;
    let c_to_f: i32 = (celsius_temp * 9/5) + 32;

    println!("Celsius to Fahrenheit: {c_to_f}");
    println!("Fahrenheit to Celsius: {f_to_c}");
}

fn fibonacci(){
    let mut counter: i32 = 0;

    let mut value_1: i32 = 0;
    let mut value_2: i32 = 1;

    while counter != 10{
        let next_fibo: i32 = value_1 + value_2;

        value_1 = value_2;
        value_2 = next_fibo;

        counter += 1;

        println!("Fibo: {next_fibo}")
    }

}