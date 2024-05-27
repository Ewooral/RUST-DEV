use std::io;

pub fn group_string_in_arr(s: &str) -> Vec<String> {
    let mut arr = Vec::new();
    let mut i: usize = 0;
    let mut in_memory_str: String = String::new();
    while i < s.len() {
        if s.len() % 2 != 0 && i == s.len() - 1 {
            in_memory_str.push_str(&s[i..i + 1]);
            in_memory_str.push_str("_");
            arr.push(in_memory_str);
            return arr;
        }
        let substring_of_two = &s[i..i + 2];
        arr.push(substring_of_two.to_string());
        i += 2;
    }
    arr
}


pub fn guessing_game() -> i32 {
    println!("Guess the number!");
    let mut guess = String::new();
    let mut value = 0;

    loop{
       println!("{}", value);
       value += 5;
       if value == 100 {
        break;
       }
    }
 value

}

