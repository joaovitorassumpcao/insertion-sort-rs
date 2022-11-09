use std::{fs::{OpenOptions}};

pub fn insertion_sort(array: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
    array.to_vec()
}

pub fn sort_file(file_path: &str){
	let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path);
	unimplemented!("sort_file");
}

#[test]
fn test_insertion_sort() {
    let mut array = vec![5, 2, 4, 6, 1, 3];
    println!("unsorted: {:?}", array);
    let sorted_array = insertion_sort(&mut array);
    println!("sorted: {:?}", sorted_array);
    assert_eq!(sorted_array, vec![1, 2, 3, 4, 5, 6]);
}
