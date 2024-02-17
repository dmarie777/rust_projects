const MY_CONSTANT: u32 = 3 * 3;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 7;
    println!("The value of x is: {x}");
    println!("{MY_CONSTANT}");
    // shadowing
    let y = 3;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is {y}");
    };
    println!("The value of y is {y}");
}
