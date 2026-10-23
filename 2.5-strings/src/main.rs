fn appendToString(input: &str) -> String {
    return String::from(input) + " appended";
}

fn main() {
    // string litural references can't be modified but ref to another string is allowed.
    let mut const_string_ref: &str = "abc";
    const_string_ref = "def";
    println!("const_string_ref={const_string_ref}");

    // heap string can be modified/extended/etc. this is the same as a std::string in C++.
    let mut heap_string = String::from("abc");

    // Can modify the bytes in the string
    unsafe {
        heap_string.as_bytes_mut()[0] = b'd';
    }
    // // and modify the characters in the string
    heap_string.replace_range(0..1, "d");

    // Concatinate to the string.
    heap_string = heap_string + "def";

    // '=' does a move operator
    println!("heap_string={heap_string}");
    let heap_string2 = heap_string;

    // heap_string is no longer valid.

    // if you want a deep copy then do:
    let heap_string3 = heap_string2.clone();

    let heap_string4 = appendToString(&heap_string3);

    println!("heap_string2={heap_string2}");
    println!("heap_string3={heap_string3}");
    println!("heap_string4={heap_string4}");
}
