fn main() {
    let mut i = 0;

    // forever loop with break that returns a value as well
    let first_div_by_seven = loop {
        i += 1;
        if i % 7 == 0 {
            break i;
        }
    };

    // For loop
    for i in 0..5 {
        println!("{i}");
    }

    // for each loop
    let a = [5, 2, 7, 2];
    for val in a {
        println!("{val}");
    }

    println!("first number divisable by seven is: {first_div_by_seven}");
}
