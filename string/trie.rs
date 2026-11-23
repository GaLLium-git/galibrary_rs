struct Node{
    children: Vec<usize>, //子
    //残りの状態
}

impl Node{
    pub fn new() -> Self{
        Self{
            children: vec![usize::MAX;26],
        }
    }
}

pub struct Trie{
    tree: Vec<Node>,
    vocab: Vec<usize>,
    tour: Vec<usize>,
    suff: Vec<usize>,
    //残りの状態 
}

impl Trie{
    pub fn new() -> Self{
        Self{
            tree: vec![Node::new()],
        }
    }
    
    pub fn insert(&mut self, word: &Vec<char>){
        let mut now = 0;
        for &c in word.iter(){
            let idx = c as usize - 'a' as usize;
            let mut nxt = self.tree[now].children[idx];
            if nxt == usize::MAX{
                nxt = self.tree.len();
                self.tree.push(Node::new());
                self.tree[now].children[idx] = nxt;
            }
            now = nxt;
            //処理
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
