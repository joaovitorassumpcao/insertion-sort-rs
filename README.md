# Insertion Sort  $\mathcal{O}(n^2)$
### Insertion sort with Rust.  
*Tecnically this isn't **true** insertion sorting as it doesn't create a new array with the sorted values, so don't tell anyone.*
  
# Implementation  
```
insertion_sort(array: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
    array.to_vec()
}
```

# Usage
```
Insertion sort on the command line.

Usage: insertion-sort [OPTIONS]

Options:
  -l, --list <list>...    Manually typed values.
  -f, --file-path <file>  Path to the file in which the contents will be sorted
  -h, --help              Print help information (use `--help` for more detail)
```
