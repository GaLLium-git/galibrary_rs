pub fn CartesianTree(A:&Vec<usize>)->Vec<usize>{
    let N = A.len();
    let mut parent = vec![usize::MAX;N];
    let mut lc = vec![usize::MAX;N]; let mut rc = vec![usize::MAX;N];

    let mut st = vec![];
    for i in 0..N{
        loop{
            if st.len() == 0{break;}
            let s = st[st.len()-1];
            if s > v{st.pop(); parent[v] = parent[s]; parent[s] = v; lc[v] = s;}
            else{break;}
        }
        st.push(A[i]);
    }

    parent
}
  
