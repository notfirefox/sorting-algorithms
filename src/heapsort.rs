fn heapify(vector: &mut Vec<i32>, h: i32, i: i32) {
    let l = (2 * i) + 1;
    let r = (2 * i) + 2;

    let mut max;
    if l < h && vector[l as usize] > vector[i as usize] {
        max = l;
    } else {
        max = i;
    }
    if r < h && vector[r as usize] > vector[max as usize] {
        max = r;
    }
    if max != i {
        vector.swap(i as usize, max as usize);
        heapify(vector, h, max);
    }
}

fn build_heap(vector: &mut Vec<i32>) {
    let h = vector.len() as i32;
    for i in (1..=((h / 2) as i32)).rev() {
        heapify(vector, h, i - 1);
    }
}

pub fn heapsort(vector: &mut Vec<i32>) {
    build_heap(vector);
    for i in (1..(vector.len() as i32)).rev() {
        vector.swap(0, i as usize);
        heapify(vector, i, 0);
    }
}
