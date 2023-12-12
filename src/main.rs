fn main() {
    // scalar type in rust
    // integer
    let _i8: i8 = 9;
    let _u8: u8 = 9;
    let _i16: i16 = 9;
    let _u16: u16 = 9;
    let _i32 = 9;
    let _u32: u32 = 9;
    let _i64: i64 = 9;
    let _u64: u64 = 9;
    let _i128: i128 = 9;
    let _u128: u128 = 9;
    let _arch: isize = 9;
    let _uarch: usize = 9;

    // integer literals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o231;
    let _binary = 0b1010_1111_1111;
    let _byte = b'A';

    // integer overflow
    // let byte: u8 = 256;
    // println!("256 in u8 is {byte}");

    // floating-point types
    let x = 2.0;
    let y: f32 = 3.0;

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 65.4 - 3.2;

    // mutiplication
    let mutiplication = 6 * 8;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    //remainder
    let remainder = 43 % 5;

    // the boolean type
    let flag = true;
    let flag: bool = false;

    // the character type
    let c = 'z';
    let c: char = 'Z';
    let heart_eyed_cat = '😻';

    // compound types
    // the tuple type
    let tup: (i32, i64, u8) = (500, 600, 2);
    let tup = (400, 6.4, 1);
    let (x, y, z) = tup;
    let four_hundrend = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // the array type
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5];

    // accessing array element
    let first = a[0];
    let second = a[1];
    let out_bound = a[100]; // index out of bounds: the length is 5 but the index is 100
}
