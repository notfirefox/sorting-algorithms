fn merge(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    let mut index_1 = 0;
    let mut index_2 = 0;

    while index_1 < vec1.len() && index_2 < vec2.len() {
        if vec1[index_1] <= vec2[index_2] {
            result.push(vec1[index_1]);
            index_1 += 1;
        } else {
            result.push(vec2[index_2]);
            index_2 += 1;
        }
    }

    if index_1 < vec1.len() {
        while index_1 < vec1.len() {
            result.push(vec1[index_1]);
            index_1 += 1;
        }
    } else if index_2 < vec2.len() {
        while index_2 < vec2.len() {
            result.push(vec2[index_2]);
            index_2 += 1;
        }
    }
    
    result
}

pub fn mergesort(vector: Vec<i32>) -> Vec<i32> {
    if vector.len() <= 1 {
        return vector;
    }

    let mid = vector.len() / 2;

    let mut vec1: Vec<i32> = vec![];
    for i in 0..mid {
        vec1.push(vector[i]);
    }

    let mut vec2: Vec<i32> = vec![];
    for i in mid..vector.len() {
        vec2.push(vector[i]);
    }

    vec1 = mergesort(vec1.clone());
    vec2 = mergesort(vec2.clone());

    merge(vec1, vec2)
}
