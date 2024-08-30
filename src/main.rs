use std::ops::IndexMut;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    dbg!(fib_std(20));
    dbg!(start.elapsed().as_nanos());

    let start = Instant::now();
    let mut memo = fib_memo();
    dbg!(memo(20));
    dbg!(start.elapsed().as_nanos());

    let start = Instant::now();
    dbg!(fib_iter(20));
    dbg!(start.elapsed().as_nanos());

    println!();

    let start = Instant::now();
    dbg!(fib_std(30));
    dbg!(start.elapsed().as_nanos());

    let start = Instant::now();
    dbg!(memo(30));
    dbg!(start.elapsed().as_nanos());

    let start = Instant::now();
    dbg!(fib_iter(30));
    dbg!(start.elapsed().as_nanos());
}

fn fib_std(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    return fib_std(n - 1) + fib_std(n - 2);
}

fn fib_memo() -> impl FnMut(u32) -> u64 {
    let mut results = Vec::<Option<u64>>::new();
    results.push(Some(0));
    results.push(Some(1));
    fn fib(vec: &mut Vec<Option<u64>>, n: usize) -> u64 {
        if vec.len() < n + 1 {
            vec.resize(n + 1, None)
        }

        if let Some(Some(res)) = vec.get(n) {
            return *res;
        }
        let res = fib(vec, n - 1) + fib(vec, n - 2);
        vec.index_mut(n).get_or_insert(res);

        res
    }

    move |n| fib(&mut results, n as usize)
}

/**
 * Fn = ( (1 + √5)^n - (1 - √5)^n ) / (2^n × √5)
 */
fn fib_iter(n: i32) -> u64 {
    (((1_f64 + 5_f64.sqrt()).powi(n) - (1_f64 - 5_f64.sqrt()).powi(n))
        / (2_f64.powi(n) * 5_f64.sqrt())) as u64
}
