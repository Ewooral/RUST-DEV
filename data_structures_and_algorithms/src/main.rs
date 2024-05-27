mod arrays;
mod stringss;

use arrays::{
    index_of_given_value, 
    my_arrays
};

use stringss::_strings::{
    group_string_in_arr,
    guessing_game
};

fn main() {
    println!("{:?}", group_string_in_arr("abcdefgh"));
    println!("{:?}", group_string_in_arr("abcdefghi"));
    println!("{:?}", group_string_in_arr("_"));
    println!("{}", index_of_given_value([1, 2, 3, 4], 8));
    println!("{}", index_of_given_value([1, 2, 3, 4], 3));
    my_arrays([1, 2, 3, 4]);
    guessing_game();
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

    #[test]
    fn test_arrays(){
        assert_eq!(my_arrays([1, 2, 3, 4]), ());
    }

    #[test]
    fn test_index_of_given_value(){
        assert_eq!(index_of_given_value([1, 2, 3, 4], 3), 2);
        assert_eq!(index_of_given_value([1, 2, 3, 4], 5), -1);
    }
}

