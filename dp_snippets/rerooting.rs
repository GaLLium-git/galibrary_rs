fn rerooting<T:Copy>(graph:&Vec<Vec<usize>>, identity:T, merge:impl Fn(T,T)->T, collect:impl Fn(usize,T)->T){
    let (depth,parent,size,index,tour) = treefs(&graph,1);
    
    let mut dp = vec![identity;graph.len()]; //普通の木dp
    for &v in tour.iter().rev(){
        for &nv in graph[v].iter(){
            if nv == parent[v] {continue;}
            dp[v] = merge(dp[v],dp[nv]);
        }
        dp[v] = collect(v,dp[v]);
    }

    let mut rdp = vec![identity;graph.len()];
    for &v in tour.iter(){
        
    }
}
