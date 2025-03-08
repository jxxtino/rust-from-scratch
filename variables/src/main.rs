fn main(){

    // mutability
    let mut x: i32 = 5; // mut: 
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("Three hours in secons: {THREE_HOURS_IN_SECONDS}");
    
    // shadowing
        // dont use this in "mut" variables
    let x: i32 = 5;

    {
        // changing x value just in this scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    //
}
