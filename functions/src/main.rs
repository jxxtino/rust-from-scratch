
fn main(){
    let x: i32 = 24;
    let y: char = 'h';

    parameters(x, y);
}

fn parameters(x: i32, y: char){
    println!(" Parameters values = x: {x} | y: {y}");

    let x: i32 = x / 4;
    let y: &str = "hours";

    println!(" Modifing parameters: x: {x} | y: {y}");
    println!(" x + 1: {}", plus_one(x));
}

fn _five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}