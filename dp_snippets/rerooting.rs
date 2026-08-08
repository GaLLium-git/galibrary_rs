fn rerooting<T:Copy>(graph:&Vec<Vec<usize>>, identity:T, merge:impl Fn(T,T)->T, collect:impl Fn(usize,T)->T) -> Vec<T>{
    let (depth,parent,size,index,tour) = treefs(&graph,1);
    
    let mut dp = vec![identity;graph.len()]; //dp[v]:部分木vの値
    let mut tmp = vec![identity;graph.len()]; //collect前
    for &v in tour.iter().rev(){
        for &nv in graph[v].iter(){
            if nv == parent[v] {continue;}
            dp[v] = merge(dp[v],dp[nv]);
        }
        tmp[v] = dp[v];
        dp[v] = collect(v,dp[v]);
    }

    let mut rdp = vec![identity;graph.len()]; //rdp[v]:部分木vを切ったときの親を根とする木の値
    for &v in tour.iter(){
        let mut acc = identity;
        for &nv in graph[v].iter(){
            if nv == parent[v] {continue;}
            rdp[nv] = acc;
            acc = merge(acc,dp[nv]);
        }
        
        let mut acc = rdp[v];
        for &nv in graph[v].iter().rev(){
            if nv == parent[v] {continue;}
            rdp[nv] = collect(v,merge(acc,rdp[nv]));
            acc = merge(acc,dp[nv]);
        }
    }
    
    let mut ans = vec![identity;graph.len()];
    for v in 0..graph.len(){
        ans[v] = collect(v,merge(tmp[v],rdp[v]));
    }
    ans
}
