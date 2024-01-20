// TODO: Consider rewriting partition method
fn partition(vector: &mut [i32], p: i32, r: i32) -> i32 {
    let x = vector[r as usize];

    let mut i = p - 1;
    for j in p..r {
        if vector[j as usize] <= x {
            i += 1;
            vector.swap(i as usize, j as usize);
        }
    }
    vector.swap((i + 1) as usize, r as usize);

    i + 1
}

pub fn quicksort(vector: &mut Vec<i32>, p: i32, r: i32) {
    if p < r {
        let q = partition(vector, p, r);
        println!("q: {q}" );
        quicksort(vector, p, q - 1);
        quicksort(vector, q + 1, r);
    }
}
