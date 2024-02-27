#[cfg(test)]
mod test;

// timeout
#[allow(dead_code)]
fn solution(arrays: Vec<Vec<i32>>) -> i32 {
    let mut max_distance = 0;

    for i in 0..arrays.len() - 1 {
        for j in i + 1..arrays.len() {
            let left_arr = &arrays[i];
            let right_arr = &arrays[j];

            let distance = (left_arr[left_arr.len() - 1] - right_arr[0]).abs();
            if distance > max_distance {
                max_distance = distance;
            }
            let distance = (left_arr[0] - right_arr[right_arr.len() - 1]).abs();
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }

    max_distance
}

#[allow(dead_code)]
fn solution2(arrays: Vec<Vec<i32>>) -> i32 {
    let mut min = arrays[0][0];
    let mut max = arrays[0][arrays[0].len() - 1];
    let mut max_distance = 0;

    for i in 1..arrays.len() {
        let distance = max - arrays[i][0];
        if distance > max_distance {
            max_distance = distance
        }
        let distance = arrays[i][arrays[i].len() - 1] - min;
        if distance > max_distance {
            max_distance = distance
        }
        if arrays[i][0] < min {
            min = arrays[i][0];
        }
        if arrays[i][arrays[i].len() - 1] > max {
            max = arrays[i][arrays[i].len() - 1];
        }
    }

    max_distance
}
