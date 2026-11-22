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
    word: Vec<usize>,
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

    pub fn search(&mut self, v:usize, word:&Vec<char>){

    }
}
