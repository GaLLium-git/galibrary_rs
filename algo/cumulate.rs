pub fn cumulate<T:Copy,F:Fn(T,T)->T>(A:&Vec<T>,f:F) -> Vec<T>{
    let mut res = A.clone();
    for i in 1..A.len(){
        res[i] = f(res[i],res[i-1]);
    }
    res
}

pub fn cumulate2D<T:Copy,F:Fn(T,T)->T>(A:&Vec<Vec<T>>,f:F) -> Vec<T>{
    let mut res = A.clone();
    
    res
}
