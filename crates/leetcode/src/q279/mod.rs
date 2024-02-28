#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(n: i32) -> i32 {
    let n = n as usize;
    let mut memo = vec![0; n + 1];

    for x in 1..=n {
        let mut min_n = i32::MAX;

        for i in 1..=(x as f32).sqrt().floor() as usize {
            min_n = min_n.min(memo[x - i * i]);
        }

        memo[x] = min_n + 1;
    }

    memo[n]
}
