//座圧
pub fn compress<T:Ord + Copy> (A:&Vec<T>) -> Vec<usize>{
    let mut uni = A.clone();
    uni.sort(); uni.dedup();
    let mut res = Vec::new();
    for i in 0..A.len(){
        res.push(bsearch_usize(0..uni.len(),|x| uni[x] >= A[i]));
    }
    res
}
