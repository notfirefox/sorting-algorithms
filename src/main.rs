mod heapsort;
mod mergesort;
mod quicksort;

use crate::heapsort::heapsort;
use crate::mergesort::mergesort;
use crate::quicksort::quicksort;

fn print_vector(prefix: &str, numbers: &Vec<i32>) {
    print!("{prefix} [");
    for i in 0..numbers.len() {
        let num = numbers[i];
        print!("{num}");
        if i < (numbers.len() - 1) {
            print!(",");
        }
    }
    println!("]");
}

fn create_unsorted() -> Vec<i32> {
    vec![1, 9, 0, 5, 6, 7, 8, 2, 4, 3]
}

fn main() {
    let numbers = create_unsorted();
    print_vector("Unsorted: ", &numbers);

    let mut numbers = create_unsorted();
    heapsort(&mut numbers);
    print_vector("Heapsort: ", &numbers);

    let mut numbers = create_unsorted();
    numbers = mergesort(numbers.clone());
    print_vector("Mergesort: ", &numbers);

    let mut numbers = create_unsorted();
    let len = numbers.len() as i32;
    quicksort(&mut numbers, 0, len - 1);
    print_vector("Quicksort: ", &numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapsort() {
        let mut numbers = vec![1, 9, 0, 5, 6, 7, 8, 2, 4, 3];
        heapsort(&mut numbers);
        assert_eq!(numbers, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_mergesort() {
        let numbers = vec![1, 9, 0, 5, 6, 7, 8, 2, 4, 3];
        let sorted = mergesort(numbers);
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quicksort() {
        let mut numbers = vec![1, 9, 0, 5, 6, 7, 8, 2, 4, 3];
        let len = numbers.len() as i32;
        quicksort(&mut numbers, 0, len - 1);
        assert_eq!(numbers, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
