//N頂点のDAG, l-rのコストf(l,r)
type T = i64;
fn Larsch(N:usize, f:impl Fn(usize,usize)->T) -> Vec<T>{
    let mut dp = vec![T::MAX;N]; dp[0] = 0; dp[N-1] = f(0,N-1);
    let mut arg = vec![0usize;N];
    
    fn solve(l:usize, r:usize, dp:&mut Vec<T>, arg:&mut Vec<usize>, f:&impl Fn(usize,usize)->T){
        if r == l+1 {return;}
        let m = (l+r)/2;
        for k in arg[l]..=arg[r]{
            (dp[m],arg[m]) = (dp[m],arg[m]).min((dp[k]+f(k,m),k));
        }
        solve(l,m,dp,arg,f);
        for k in l..=m{(dp[r],arg[r]) = (dp[r],arg[r]).min((dp[k]+f(k,r),k));}
        solve(m,r,dp,arg,f);
    }
    
    solve(0,N-1,&mut dp,&mut arg,&f);
    dp
}
