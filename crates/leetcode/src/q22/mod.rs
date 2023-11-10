use std::collections::HashSet;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut memo: Vec<HashSet<String>> = vec![];

    for i in 0..=n {
        let mut cases = HashSet::<String>::new();

        if i == 1 {
            cases.insert("()".to_string());
            memo.push(cases);
            continue;
        }

        for j in 0..i {
            if j == 0 {
                memo[i - 1].iter().for_each(|x| {
                    cases.insert(format!("(){}", x));
                });
                continue;
            }

            if j == i - 1 {
                memo[i - 1].iter().for_each(|x| {
                    cases.insert(format!("({})", x));
                });
                continue;
            }

            memo[j].iter().for_each(|x| {
                memo[i - j - 1].iter().for_each(|y| {
                    cases.insert(format!("({}){}", x, y));
                });
            });
        }

        memo.push(cases);
    }

    memo[n].drain().collect()
}
