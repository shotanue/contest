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

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

fn primes(num: i64) -> Vec<i64> {
    let max = (num as f64).sqrt() as i64 + 1;
    let mut i = 2;
    let mut ans = vec![1];
    let mut now = num;
    while max >= i {
        let res = now % i;
        if res == 0 {
            now = now / i;
            ans.push(i);
        } else {
            i += 1;
        }
    }
    ans.dedup();
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

