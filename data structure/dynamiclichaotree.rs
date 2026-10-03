fn main() {
    let mut sc = Scanner::new();
    let (N,C):(usize,i64) = (sc.next(),sc.next());
    let mut h:Vec<i64> = (0..N).map(|_| sc.next()).collect();
    let mut dp = vec![0i64;N];
    
    type F = (usize,i64);
    let eval = |f:F,x:Tx| -> Ty{
        if f.0 == usize::MAX{i64::MAX}
        else {f.1+(h[f.0]-h[x])*(h[f.0]-h[x])+C}
    };
    
    let mut lichao = LiChaoTree::new(eval,(usize::MAX,0),0,N);
    lichao.add((0,0));
    for i in 1..N{
        dp[i] = lichao.get(i);
        lichao.add((i,dp[i]));
    }
    //eprintln!("{:?}",dp);
    println!("{}",dp[N-1]);
}


type Tx = usize; //引数の型
type Ty = i64; //返り値の型
#[derive(Copy, Clone)]
struct Node<F>{
    f: F,
    lc: Option<usize>,
    rc: Option<usize>,
}

#[derive(Clone)]
pub struct LiChaoTree<F,Eval>
where
    F: Copy,
    Eval: Fn(F, Tx) -> Ty,
{
    tree: Vec<Node<F>>,    
    eval: Eval,
    id: F,
    x_min: Tx,
    x_max: Tx,
}

impl<F,Eval> LiChaoTree<F,Eval>
where
    F: Copy,
    Eval: Fn(F, Tx) -> Ty,
{
    pub fn new(eval:Eval, id:F, x_min:Tx, x_max:Tx) -> Self{
        Self {
            tree: vec![Node{f:id,lc:None,rc:None}],
            eval,
            id,
            x_min,
            x_max,
        }
    }
    
    //[l,r)を覆うノードvでの更新
    fn _add(&mut self, mut f:F, v:usize, l:Tx, r:Tx){
        let m = l+(r-l)/2;
        let new_m = (self.eval)(f,m);
        let pre_m = (self.eval)(self.tree[v].f,m);
        
        if new_m < pre_m{
            (self.tree[v].f,f) = (f,self.tree[v].f);
        }
        
        if r - l == 1{return;}
        
        let new_l = (self.eval)(f,l);
        let new_r = (self.eval)(f,r-1);
        let pre_l = (self.eval)(self.tree[v].f,l);
        let pre_r = (self.eval)(self.tree[v].f,r-1);
        
        if new_l >= pre_l && new_r >= pre_r{return;}
        
        else if new_l < pre_l{
            if self.tree[v].lc.is_none(){
                self.tree[v].lc = Some(self.tree.len());
                self.tree.push(Node{f:self.id,lc:None,rc:None});
            }
            self._add(f, self.tree[v].lc.unwrap(), l, m);
        }
        
        else if new_r < pre_r{
            if self.tree[v].rc.is_none(){
                self.tree[v].rc = Some(self.tree.len());
                self.tree.push(Node{f:self.id,lc:None,rc:None});
            }
            self._add(f, self.tree[v].rc.unwrap(), m, r);
        }
        
    }
    
    pub fn add(&mut self, f:F){
        self._add(f,0,self.x_min,self.x_max);
    }
    
    //[l,r)を覆うノードvでのｘの代入
    fn _get(&self, x:Tx, v:usize, l:Tx, r:Tx) -> Ty{
        let m = l+(r-l)/2;
        let mut res = (self.eval)(self.tree[v].f, x);
        //左側
        if l <= x && x < m && self.tree[v].lc.is_some(){
            res = res.min(self._get(x,self.tree[v].lc.unwrap(),l,m));
        }
        //右側
        if m <= x && x < r && self.tree[v].rc.is_some(){
            res = res.min(self._get(x,self.tree[v].rc.unwrap(),m,r));
        }
        res
    }
    
    pub fn get(&self, x:Tx) -> Ty{
       self._get(x,0,self.x_min,self.x_max)
    }
}
