#[allow(dead_code)]
fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

#[allow(dead_code)]
fn primes(num: i64) -> Vec<i64> {
    let mut ans = vec![1];
    let mut cur = num;
    for i in 2.. {
        if i * i > num { break; }
        if cur % i == 0 {
            while cur % i == 0 {
                ans.push(i);
                cur /= i;
            }
        }
    }
    if cur > 1 {
        ans.push(cur);
    }
    return ans;
}

#[test]
fn test_primes() {
    assert_eq!(primes(1), vec![1]);
    assert_eq!(primes(140), vec![1, 2, 2, 5, 7])
}

// https://techracho.bpsinc.jp/jhonda/2019_08_05/78537
macro_rules! test {
    ($($input:expr => $output:expr),* $(,)*) => {
        #[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input), $output);
            )*
        }
    };
}
test! {
}

use proconio::{input, fastout};
use proconio::source::auto::AutoSource;

fn solve(src: &str) -> String {
    let source = AutoSource::from(src);
    input! {
       from source,
       s:String,
    }
    let s: String = s;
    let ans = s;

    format!("{}", ans)
}


#[fastout]
fn main() {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    println!("{}", solve(&*s));
}
