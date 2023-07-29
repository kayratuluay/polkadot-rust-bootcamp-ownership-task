
fn main() {
    let string1 = String::from("I really");
    let string2 = String::from(" liked Rust!");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}",concatenated_string);
}

fn concatenate_strings(string_one:&str, string_two:&str) -> String{
    let mut result = String::from(string_one);
    result.push_str(string_two);
    result
}
