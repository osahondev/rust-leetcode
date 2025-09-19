fn main() {
    println!("Welcome to the program that solves the problem for longest common prefix!");
    dbg!(longest_common_prefix(vec![
        String::from("ab"),
        String::from("a"),
        // String::from("flight")
    ]));
}
fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::from("");
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }
    let first_element = &strs[0];
    let mut ptr_index = 0;
    for (index, element) in first_element.chars().enumerate() {
        for str in &strs[1..] {
            if index >= str.len(){
                return String::from(&first_element[0..index]);
            }  
            let char = &str[index..index + 1];
            if element.to_string() != char.to_string() {
                return String::from(&first_element[0..index]);
            }
        }
        ptr_index += 1;
    }
    String::from(&first_element[0..ptr_index])
}
