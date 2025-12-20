// Use &str over &String so can be used for string liturals and String objects
fn first_word(input: &str) -> &str {
    let charactors = input.as_bytes();
    let mut end = 0;

    for i in 0..charactors.len() {
        if charactors[i] == b' ' {
            end = i;
            break;
        }
    }

    if end == 0 {
        end = input.len();
    }

    let sub_string = &input[..end];
    return sub_string;
}

fn main() {
    let result = first_word("hello world");
    println!("result = {result}")
}
