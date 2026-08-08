let rerooting = {
        let mut (depth,parent,size,index,tour) = treefs(&graph,1);
        let mut ans = vec![];
    
    
        let identity = 1usize;
        let mut merge = |x:usize,y:usize| -> usize {(x+y)%M};
        let mut collect = |v:usize,x:usize| -> usize {(x+1)%M};
    
        let mut dp = vec![identity;N+1]; //普通の木dp
        for &v in tour.iter().rev(){
            for &nv in graph[i].len(){
                if nv == parent[v] {continue;}
                dp[v] = merge(dp[v],dp[nv]);
            }
            dp[v] = collect(v,dp[v]);
        }
    
        let mut rdp = vec![identity;N+1];
    }
