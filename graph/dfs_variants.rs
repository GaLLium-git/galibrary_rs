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


//木の色々
pub fn treesis(graph:&Vec<Vec<usize>>, root:usize) -> (Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>){
    let mut tour = vec![];
    let mut index = vec![usize::MAX;graph.len()];
    let mut size = vec![usize::MAX;graph.len()];

    let mut dfs = recur_fn();
    dfs(root,root);
    (tour,index,size)
}
