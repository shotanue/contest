// stdin
macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

// input
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

// Test
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

#[allow(dead_code)]
fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

#[test]
fn test_primes() {
    assert_eq!(primes(1), vec![1]);
    assert_eq!(primes(140), vec![1, 2, 2, 5, 7])
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


test! {
}

fn solve(src: &str) -> String {
    input! {
        source=src,
//        n:usize,
//        a:[i32;n],
    }
    let ans = "";
    format!("{}", ans)
}


fn main() {
    println!("{}", solve(&stdin!()));
}

