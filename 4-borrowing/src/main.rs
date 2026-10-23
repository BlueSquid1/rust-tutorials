// Rust assignment operator by default a move operator (i.e. shallow copy and mark the original as invalid)
// only exception is if the data type implements the copy trait
// can't add a copy trait if the data type implements the Drop trait (a.k.a. it's destructor)

// std::unique_ptr<std::string> append_to_string(const std::unique_ptr<std::string>& input)
fn append_to_string(input: &String) -> String {
    //return std::make_unique<std::string>(*input + " world");
    return String::from(input) + " world";
}

// std::unique_ptr<std::string> append_to_string(std::unique_ptr<std::string> input)
fn append_to_string_and_move(input: String) -> String {
    //return std::make_unique<std::string>(*input + " world");
    return String::from(input) + " world";
}

fn main() {
    // std::unqiue_ptr<std::string> a = std::make_unique<std::string>("hello");
    let mut a = String::from("hello");

    // const std::unqiue_ptr<std::string> b = std::make_unique<std::string>(*a);
    let b = a.clone();

    a.clear();

    println!("after deep copy a is {a} and b is: {b}");

    // std::unique_ptr<std::string> b = std::move(a);
    let b = a;

    // can't use a anymore because it has been moved to b.
    println!("after move a is 'invalid' and b is: {b}");

    // auto hello = std::make_unique<std::string>("hello");
    let hello = String::from("hello");

    // auto message = append_to_string(hello);
    let message = append_to_string(&hello);
    println!("message = {message}");

    // auto message = append_to_string_and_move(std::move(hello));
    let message = append_to_string_and_move(hello);
    println!("message = {message}");
}
