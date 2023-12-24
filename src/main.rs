// mod heapsort;
mod mergesort;
// mod quicksort;

// use crate::heapsort::heapsort;
use crate::mergesort::mergesort;
// use crate::quicksort::quicksort;

fn print_vector(numbers: &Vec<i32>) {
    print!("[ ");
    for i in 0..numbers.len() {
        let num = numbers[i];
        print!("{}", num);
        if i < (numbers.len() - 1) {
            print!(", ");
        }
    }
    println!(" ]")
}

fn main() {
    let mut numbers = vec![1, 9, 0, 5, 6, 7, 8, 2, 4, 3];

    /*
    print_vector(&numbers);
    heapsort(&mut numbers);
    print_vector(&numbers);
    */

    print_vector(&numbers);
    numbers = mergesort(numbers.clone());
    print_vector(&numbers);

    /*
    let len = numbers.len() as i32;
    print_vector(&numbers);
    quicksort(&mut numbers, 0, len - 1);
    print_vector(&numbers);
    */
}
