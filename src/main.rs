fn main() {
    let mut s = String::from("Hello world");

    let len = calculate_len(&s);

    println!("the length of '{}' is {}", s, len);

    change(&mut s);

    println!("{s}");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1} {r2}");

    let r1 = &mut s;
    println!("{r1}");
    let r2 = &mut s;
    println!("{r2}");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{r1}{r2}{r3}");

    let r1 = &s;
    let r2 = &s;
    println!("{r1}{r2}");
    let r3 = &mut s;
    println!("{r3}");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str((", i am rust"));
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn dangle_save() -> String {
    let s = String::from("hello");
    s
}
