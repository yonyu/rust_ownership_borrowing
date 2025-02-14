// use struct Shuttle defined in another file 'struct_lifetime_annotations' in the same project
mod struct_lifetime_annotations;

use struct_lifetime_annotations::Shuttle;

fn main() {
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("Liquid Hydrogen");

    let result = best_fuel(&propellant1, &propellant2);
    println!("The best fuel is {}", result);

    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Mission Control, we have liftoff!");
    println!("Sender is {}", sender);

    // // println!("Hello, world!");
    // let mut x: i32 = 42;

    // let y: &mut i32 = &mut x;
    // *y += 1;

    // //println!("x = {}", x);
    // println!("y = {}", y);
}

fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct Item {
    pub weight: i32,
    pub value: i32,
}

// implement knapsack with memoization, that is, top-down dynamic programming
pub fn knapsack_memoization(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut memo = vec![vec![-1; capacity as usize + 1]; n + 1];
    // for i in 0..=n {
    //     memo[i][0] = 0;
    // }

    // for j in 1..=capacity {
    //     memo[0][j as usize] = 0;
    // }

    fn knapsack_memoization_helper(items: &Vec<Item>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        // base case
        if i == 0 || j == 0 {
            return 0;
        }

        // if the value is already computed, return it
        if memo[i][j] != -1 {
            return memo[i][j];
        }

        let mut max1 = knapsack_memoization_helper(items, i - 1, j, memo);
        if j >= items[i - 1].weight as usize {
            let max2 = knapsack_memoization_helper(items, i - 1, j - items[i - 1].weight as usize, memo);
            max1 = max1.max(max2 + items[i - 1].value);
        }

        memo[i][j] = max1;

        memo[i][j]
    }

    let max_value: i32 = knapsack_memoization_helper(&items, n, capacity as usize, &mut memo);

    // backtrack all items included in the knapsack, starting from the last item
    let mut packed: Vec<i32> = Vec::new();
    let mut i = n;
    let mut j = capacity as usize;
    while i > 0 {
        if memo[i][j] != memo[i - 1][j] {
            packed.push(i as i32);
            j -= items[i - 1].weight as usize;
        }
        i -= 1;
    }

    packed.reverse();
    (max_value, packed)
}

pub fn knapsack(items: &Vec<Item>, capacity: i32) -> (i32, Vec<i32>) {
    let n = items.len();
    let mut dp = vec![vec![-1; 1 + capacity as usize]; n + 1];

    for i in 0..=n {
        dp[i][0] = 0;
    }

    for j in 0..=capacity {
        dp[0][j as usize] = 0;
    }

    for i in 1..=n {
        for j in 1..=capacity {
            if items[i - 1].weight > j {
                dp[i][j as usize] = dp[i - 1][j as usize];
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize]
                    .max(items[i - 1].value + dp[i - 1][j as usize - items[i - 1].weight as usize]);
                //let t = i as i32;
                // if !packed.contains(&t) {
                //     packed.push(i as i32);
                // }
            }
        }
    }

    // backtrack all items included in the knapsack, starting from the last item
    let mut packed: Vec<i32> = Vec::new();
    let mut j = capacity as usize;
    let mut i = n;
    while i > 0 {
        if dp[i][j] != dp[i - 1][j] {
            packed.push(i as i32);
            j -= items[i - 1].weight as usize;
        }
        i -= 1;
    }

    packed.reverse();

    (dp[n][capacity as usize], packed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack() {
        let mut items: Vec<Item> = Vec::new();

        let item = Item {
            weight: 2,
            value: 12,
        };
        items.push(item);

        let item = Item {
            weight: 1,
            value: 10,
        };
        items.push(item);

        let item = Item {
            weight: 3,
            value: 20,
        };
        items.push(item);

        let item = Item {
            weight: 2,
            value: 15,
        };
        items.push(item);

        let capacity = 5;

        let result = knapsack(&items, capacity);

        assert_eq!(result.0, 37);
        assert_eq!(result.1, [1, 2, 4]);

        let result = knapsack_memoization(&items, capacity);
        assert_eq!(result.0, 37);
        assert_eq!(result.1, [1, 2, 4]);
    }
}
