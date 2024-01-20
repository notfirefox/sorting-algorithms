fn heapify(vector: &mut Vec<i32>, h: usize, i: usize) {
    let l = (2 * i) + 1;
    let r = (2 * i) + 2;

    let mut max = if l < h && vector[l] > vector[i] { l } else { i };
    if r < h && vector[r] > vector[max] {
        max = r;
    }
    if max != i {
        vector.swap(i, max);
        heapify(vector, h, max);
    }
}

fn build_heap(vector: &mut Vec<i32>) {
    let h = vector.len();
    for i in (1..=(h / 2)).rev() {
        heapify(vector, h, i - 1);
    }
}

pub fn heapsort(vector: &mut Vec<i32>) {
    build_heap(vector);
    for i in (1..(vector.len())).rev() {
        vector.swap(0, i);
        heapify(vector, i, 0);
    }
}
