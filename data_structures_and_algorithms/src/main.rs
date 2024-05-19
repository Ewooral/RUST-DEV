pub fn group_string_in_arr(s: &str) -> Vec<String> {
    let mut arr = Vec::new();
    let mut i = 0;
    let mut in_memory_str: String = String::new();
    while i < s.len() {
        if s.len() % 2 != 0 && i == s.len() - 1 {
            let first_char = s.chars().nth(i);
            println!("char:: {:?}", first_char );
            in_memory_str.push_str(&s[i..i+1]);
            in_memory_str.push_str("_");
            arr.push(in_memory_str);
            break;
        }
        let substring_of_two = &s[i..i+2];
        arr.push(substring_of_two.to_string());
        i += 2;
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_string_in_arr() {
        assert_eq!(group_string_in_arr("hello"), vec!["he", "ll", "o_"]);
        assert_eq!(group_string_in_arr("world"), vec!["wo", "rl", "d_"]);
        assert_eq!(group_string_in_arr("python"), vec!["py", "th", "on"]);
        assert_eq!(group_string_in_arr("rust"), vec!["ru", "st"]);
        assert_eq!(group_string_in_arr("a"), vec!["a_"]);
        assert_eq!(group_string_in_arr(""), Vec::<String>::new());
        
    }
}

fn main() {
    println!("{:?}", group_string_in_arr("abcdefgh"));
    println!("{:?}", group_string_in_arr("abcdefghi"));
    println!("{:?}", group_string_in_arr(""));
}