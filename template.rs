// https://github.com/GaLLium-git/galibrary_rs/blob/main/template.rs
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use template::*;
use itertools::*;
use std::collections::*;

/* 考察

*/
fn main() {
    let mut sc = Scanner::new();
    let mut ans = 0;
    println!("{}",ans);
}


pub mod template{
    
//Scanner
pub struct Scanner{
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self{
        Scanner {buffer: std::collections::VecDeque::new()}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T{
        if self.buffer.len() == 0{
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().map(|s| s.to_string()).collect();
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn next_chars(&mut self) -> Vec<char>{
        self.next::<String>().chars().collect()
    }
}

//range型を[l,r)に直す関数
pub fn get_bounds_usize(range: impl std::ops::RangeBounds<usize>) -> (usize,usize){
    let l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => *l+1,
        std::ops::Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        std::ops::Bound::Included(r) => *r+1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    (l,r)
}

//二分探索 range で fがtrueとなる最小を返す
pub fn bsearch_usize<F>(range: impl std::ops::RangeBounds<usize>, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut l, mut r) = get_bounds_usize(range);
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

//累積和(1次元,2次元)
pub fn cumulate<T:Copy,F:Fn(T,T)->T>(A:&Vec<T>,f:F) -> Vec<T>{
    let mut res = A.clone();
    for i in 1..A.len(){
        res[i] = f(res[i],res[i-1]);
    }
    res
}
pub fn cumulate2D<T:Copy,F:Fn(T,T)->T>(A:&Vec<Vec<T>>,f:F) -> Vec<T>{
    let mut res = A.clone();
    for i in 0..A.len(){
        for j in 1..A[0].len(){
            res[i][j] = f(res[i][j],res[i][j-1]);
        }
    }
    for j in 0..A[0].len(){
        for j in 1..A.len(){
            res[i][j] = f(res[i][j],res[i-1][j]);
        }
    }
    res
}
}
