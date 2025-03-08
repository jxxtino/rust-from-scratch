fn main() {

    // integer
        // for signed (i) NEGATIVES & POSITIVES| unsigned (u) POSITIVES
    let x: u32 = 232; 
    let y: i8 = -5; 

    println!( "Integers = x: {x} | y: {y}" );
    
    // floating-point 
        // f64: 64 bits size (default) | f32: 32 bits size
    let x: f64 = 2.2;
    let y: f32 = 1.4;
    
    println!("Floating = x: {x} | y: {y}");

    // numeric operations
        // addition
    let sum: u8 = 5 + 7;
        // subtraction
    let difference: i8 = 10 - 32;
        // multiplication
    let product: i16 = 87 * 42;
        // division
    let quotient: u8 = 10 / 5;
    let _truncated: i8 = -13 / 3;
        // remainder
    let remainder: u8 = 45 % 5;
    println!("Add: {sum} | Sub: {difference} | Mult: {product} | Div: {quotient} | Rem: {remainder}");

    // boolean
    let _a: bool = true;

    // character 
        // char types can represent a lot more than just ASCII, example= '😻'
    let _c: char = 'C';
    let emoji: char = '😻';
    println!("{emoji}");

    // compound types (group multiple values)
        // tuple
        let tup: (u8,f64,u8) = (5,10.0,1);
        let (x,_y,z) = tup;
        
        println!("x: {x} | z: {z}");
            // acessing a tuple by index
        let first_value = tup.0;
        println!("Firts Value: {first_value}");

        // array
            // fixed size [5]| same type: [i32:]
        let array_example: [i32; 5] = [1, 2, 3, 4, 5];
        println!("{:?}", array_example); // {:?}: shows the entire array
        println!("last item: {}", array_example[4]);

        let array_same_value = [6,3];
        println!("{:?}", array_same_value)

}
