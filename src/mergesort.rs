fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    match (left, right) {
        ([], []) => [].to_vec(),
        (_, []) => left.to_vec(),
        ([], _) => right.to_vec(),
        (_, _) => {
            let (x, xs) = left.split_first().expect("Left was empty.");
            let (y, ys) = right.split_first().expect("Right was empty.");

            if x < y {
                [vec![*x], merge(xs, right)].concat()
            } else {
                [vec![*y], merge(left, ys)].concat()
            }
        }
    }
}

pub fn mergesort(vector: Vec<i32>) -> Vec<i32> {
    if vector.len() <= 1 {
        return vector;
    }

    let mid = vector.len() / 2;

    let (left, right) = vector.split_at(mid);
    let vec1 = mergesort(left.to_vec());
    let vec2 = mergesort(right.to_vec());

    merge(&vec1, &vec2)
}
