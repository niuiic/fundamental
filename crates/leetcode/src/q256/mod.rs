#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(costs: Vec<Vec<i32>>) -> i32 {
    let mut memo = [0; 3];

    for i in 0..costs.len() {
        let mut temp = [0; 3];
        let cost = &costs[i];

        for j in 0..cost.len() {
            for k in 0..memo.len() {
                if j == k {
                    continue;
                }
                if temp[j] == 0 || temp[j] > cost[j] + memo[k] {
                    temp[j] = cost[j] + memo[k];
                }
            }
        }

        memo = temp;
    }

    *memo.iter().min().unwrap()
}
