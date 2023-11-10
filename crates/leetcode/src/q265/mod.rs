#[cfg(test)]
mod test;

#[allow(dead_code)]
pub fn solution(costs: Vec<Vec<i32>>) -> i32 {
    let color_count = costs[0].len();
    let mut memo = vec![0; color_count];

    for i in 0..costs.len() {
        let mut temp = vec![0; color_count];
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
