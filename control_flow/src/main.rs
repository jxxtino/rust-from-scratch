fn main(){
    let number: i32 = 5;

    if number > 10{
        println!(" Condition was true");
    } else {
        println!(" Condition was false ");
    }

        //  checking variables
    let is_empty: bool = false;

    if is_empty{
        println!(" The box is not empty");
    } else {
        println!(" The box is empty");
    }

        // multi conditions
    let number: i32 = 34;

    if number % 2 == 0 {
        println!(" Number: {number} is divisible by 2")
    } else if number % 3 == 0 {
        println!(" Number: {number} is divisible by 3")
    } else if number % 4 == 0 {
        println!(" Number: {number} is divisible by 4")
    } else {
        println!(" Number: {number} isn't divisible by 4, 3 or 2")
    }

        // if and let
    let condition: bool = true;
    let decision: i32 = if condition {5} else {6}; // values must hava same type!!

    println!(" Choice: {decision}");


    loops();
}

fn loops(){
    // basic loop (infinite)
    loop {
        println!("again!!");
        break;
    }

    let mut counter: i32 = 0;

    let result = loop{
        counter += 1;
        println!("Counter: {counter}");

        if counter == 10{
            break counter;
        }
    };

    println!("The result is {result}");

    loops_label();
}

fn loops_label() {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");
        
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
        
            if remaining == 9 {
                break;
            }
        
            if count == 2 {
                break 'counting_up;
            }
        
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    loops_while()
}

fn loops_while(){
    let mut index: i32 = 3;

    while index != 0 {
        println!("index^: {index}");

        index -= 1;
    }

    println!("loop out");
}