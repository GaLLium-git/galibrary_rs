//いろんなdfs

//seenを持つやつ
pub fn dfs(graph:&Vec<Vec<usize>>, v:usize, seen:&mut Vec<bool>){
    seen[v]=true;
    for &nv in graph[v].iter(){
        if !(seen[nv]) {
          dfs(graph,nv,seen);
        }
    }
}

//親を持つやつ
pub fn dfs(graph:&Vec<Vec<usize>>, v:usize, p:usize){
    for &nv in graph[v].iter(){
        if nv == p {continue;}
        dfs(graph,nv,v);
    }
}


//(depth,parent,size,index,tour)
pub fn treefs(graph:&Vec<Vec<usize>>, root:usize) -> (Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>){
    let mut depth = vec![usize::MAX;graph.len()];
    let mut parent = vec![usize::MAX;graph.len()];
    let mut size = vec![usize::MAX;graph.len()];
    let mut index = vec![usize::MAX;graph.len()];
    let mut tour = vec![];
    
    fn dfs(graph:&Vec<Vec<usize>>, v:usize, p:usize, depth:&mut Vec<usize>, parent:&mut Vec<usize>, size:&mut Vec<usize>, index:&mut Vec<usize>, tour:&mut Vec<usize>){
        index[v] = tour.len(); 
        tour.push(v);
        parent[v] = p;
        if p!=usize::MAX {depth[v] = depth[p]+1;}
        let mut sz = 1usize;
        for &nv in graph[v].iter(){
            if nv == p {continue;}
            dfs(graph,nv,v,depth,parent,size,index,tour);
            sz += size[nv];
        }
        size[v] = sz
    }
    
    dfs(graph,root,usize::MAX,&mut depth,&mut parent,&mut size,&mut index,&mut tour);
    (depth,parent,size,index,tour)
}
