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
