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
    
    let mut dfs = recur_fn(|v:usize,p:usize|{
        if p != usize::MAX
        tour.push(v);
        
    });
    dfs(root,usize::MAX);
    (tour,index,size)
}
