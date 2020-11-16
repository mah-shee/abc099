#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut res = n;
    for i in 0..=n {
        let mut cc = 0;
        let mut t = i;
        while t > 0 {
            cc += t % 6;
            t /= 6;
        }
        t = n - i;
        while t > 0 {
            cc += t % 9;
            t /= 9;
        }
        if res > cc {
            res = cc;
        }
    }
    println!("{}", res);
}
