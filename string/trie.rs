struct Node{
    child: Vec<usize>, //子
    //残りの状態
}

impl Node{
    pub fn new() -> Self{
        Self{
            child: vec![usize::MAX;26],
        }
    }
}

pub struct Trie{
    tree: Vec<Node>,
    ord: Vec<usize>,
    tour: Vec<usize>, //bfs
    parent: Vec<usize>,
    suff: Vec<usize>,
    //残りの状態 
}

impl Trie{
    pub fn new() -> Self{
        Self{
            tree: vec![Node::new()],
            ord: vec![],
            tour: vec![],
            parent: vec![0usize],
            suff: vec![],
        }
    }
    
    pub fn insert(&mut self, word: &Vec<char>){
        let mut now = 0;
        for &c in word.iter(){
            let idx = c as usize - 'a' as usize;
            let mut nxt = self.tree[now].child[idx];
            if nxt == usize::MAX{
                nxt = self.tree.len();
                self.tree.push(Node::new());
                self.tree[now].child[idx] = nxt;
                self.parent.push(now);
            }
            now = nxt;
            //処理
        }
        self.ord.push(now);
    }


    pub fn ahoc(&mut self){
        let mut que = std::collections::VecDeque::new(); que.push_back(0);
        self.suff = vec![0;self.tree.len()];
        while !que.is_empty(){
            let v = que.pop_front().unwrap();
            self.tour.push(v);
            for c in 0..26{
                let nv = self.tree[v].child[c];
                if nv == usize::MAX{
                    self.tree[v].child[c] = if v == 0 {0} else{self.tree[self.suff[v]].child[c]};
                }else{
                    self.suff[nv] = if v == 0 {0} else{self.tree[self.suff[v]].child[c]};
                    que.push_back(nv);
                }
            }
        }
            
    }
        
}


    pub fn ahoc(&mut self){
        let mut que = VecDeque::new();
        for &c in 0..26{
            if self.tree[0][c] == usize::MAX{
                self.tree[0][c] = 0;
            }else{
                self.suff[self.tree[0][c] = 0;
                que.push_back(self.tree[0][c]);
            }
        }
        while !que.empty(){
            let v = que.pop_front();
            for &c in 0..26{
                let nv = self.tree[v][c];
                if nv == usize::MAX{
                    self.tree[v][c] = self.tree[suff[v]][c];
                }else{
                    self.suff[nv] = self.tree[self.suff[v]][c];
                    que.push_back(nv);
                }
            }
        }
            
    }
        
}
