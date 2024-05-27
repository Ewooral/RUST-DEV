// ARRAYS
pub fn my_arrays(arr: [i32; 4]) {
    let mut i = 0;
    while i < arr.len() as i32 {
        println!("{:?}", i);
        i += 1
    }
}

pub fn index_of_given_value(arr: [i32; 4], value: i32) -> i32 {
    let mut i: i32 = 0;
    while i < arr.len() as i32 {
        if arr[i as usize] == value {
            return i;
        }
        i += 1;
    }
    -1
}

