fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("constant is {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("the value of y in the inner scope is: {y}");
    }
    println!("the value of y is {y}");

    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces = "   ";
    spaces = spaces.len();
}
