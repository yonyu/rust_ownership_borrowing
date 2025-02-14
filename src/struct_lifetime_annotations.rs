pub struct Shuttle<'a> {
    pub name: &'a str,
}

// impl<'a> Shuttle<'a> {
//     pub fn send_transmission(&self, msg: &str) -> &str {
//         println!("{}: {}", self.name, msg);
//         self.name
//     }
// }

impl<'a, 'b> Shuttle<'a> {
    pub fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("{}: {}", self.name, msg);
        msg
    }
}

#[allow(dead_code)]
///generate the function to do substraction
fn substract(x: i32, y: i32) -> i32 {
    x - y
}

//generate a function to calculate coin change with dynamic programming
#[allow(dead_code)]
fn coin_change(coins: Vec<i32>, n: i32) -> i32 {
    let mut dp = vec![n + 1; n as usize + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut temp: i32 = n + 1;
        for &coin in &coins {
            if i >= coin {
                //temp = std::cmp::min(temp, dp[ (i - coin) as usize]);
                temp = std::cmp::min(temp, dp[(i - coin) as usize]);
            }
        }
        dp[i as usize] = temp + 1i32;
    }
    dp[n as usize]
}

#[allow(dead_code)]
fn coin_change_2(coins: Vec<i32>, n: i32) -> i32 {
    let mut dp = vec![n + 1; n as usize + 1 ];
    dp[0] = 0;
    for i in 1..=n {
        for coin in &coins {
            if i - coin >= 0 {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
        //dp[i as usize] += 1;
    }
    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substract() {
        assert_eq!(substract(5, 3), 2);
    }

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![1, 2, 5], 5), 1);
    }

    #[test]
    fn test_coin_change_2() {
        assert_eq!(coin_change(vec![1, 3, 4], 6), 2);
    }

    #[test]
    fn test_coin_change_3() {
        assert_eq!(coin_change_2(vec![1, 3, 4], 6), 2);
    }
}
