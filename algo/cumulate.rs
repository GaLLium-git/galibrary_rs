pub fn cumulate<T:Copy>(A:&Vec<T>) -> Vec<T>{
    let mut res = A.clone();
    for i in 1..A.len(){
        res[i] = f(res[i],res[i-1]);
    }
    res
}
