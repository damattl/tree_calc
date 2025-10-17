#[derive(Clone, Debug)]
pub struct BinaryTree<T> {
    mem: Vec<T>,
    empty: T,
}

impl<T> BinaryTree<T>
where
    T: Clone + PartialEq,
{
    pub fn new(empty: T) -> Self {
        BinaryTree::<T> {
            mem: Vec::new(),
            empty,
        }
    }
    pub fn new_with_root(x: T, empty: T) -> Self {
        BinaryTree::<T> {
            mem: vec![x],
            empty,
        }
    }
    pub fn bin(&mut self, x: T, b: BinaryTree<T>) -> &BinaryTree<T> {
        let mut mem = vec![x];

        let new_len = self.mem.len().max(b.mem.len()) * 2 + 1;

        for i in 1..new_len {
            let mut mask = (i + 1).next_power_of_two();
            if !(i + 1).is_power_of_two() {
                mask = mask >> 1;
            }
            // println!("i = {}", i);
            // println!("mask = {:b}", mask);

            if (i + 1) ^ mask >= mask >> 1 {
                let idx = i ^ mask;
                match b.mem.get(idx) {
                    Some(x) => mem.push(x.clone()),
                    None => mem.push(self.empty.clone()),
                };
                // println!("b");
            } else {
                // println!("mask >> 1 = {:b}", mask >> 1);
                // println!("mask & mask = {:b}", mask | (mask >> 1));
                // println!("i + 1 = {:b}", i + 1);
                let idx = ((i + 1) ^ (mask | (mask >> 1))) - 1;
                match self.mem.get(idx) {
                    Some(x) => mem.push(x.clone()),
                    None => mem.push(self.empty.clone()),
                };
                // println!("self")
            }
        }

        self.mem = mem;
        self
    }

    pub fn left_idx(k: usize) -> usize {
        2 * k + 1
    }

    pub fn right_idx(k: usize) -> usize {
        2 * k + 2
    }

    pub fn parent_idx(k: usize) -> Option<usize> {
        if k == 0 {
            return None;
        }
        Some((k - 1) / 2)
    }

    pub fn left(&self, k: usize) -> (Option<usize>, Option<&T>) {
        let idx = BinaryTree::<T>::left_idx(k);
        match self.mem.get(idx) {
            Some(node) => match *node == self.empty {
                true => (None, None),
                false => (Some(idx), Some(node)),
            },
            None => (None, None),
        }
    }

    pub fn right(&self, k: usize) -> (Option<usize>, Option<&T>) {
        let idx = BinaryTree::<T>::right_idx(k);
        match self.mem.get(idx) {
            Some(node) => match *node == self.empty {
                true => (None, None),
                false => (Some(idx), Some(node)),
            },
            None => (None, None),
        }
    }

    pub fn parent(&self, k: usize) -> (Option<usize>, Option<&T>) {
        let idx = BinaryTree::<T>::parent_idx(k);
        match idx {
            Some(idx) => (Some(idx), self.mem.get(idx)),
            None => (None, None),
        }
    }

    pub fn traverse_postorder<F>(&self, k: usize, f: &mut F)
    where
        F: FnMut((usize, Option<&T>)),
    {
        if k >= self.mem.len() {
            return;
        }
        let node = self.mem.get(k);
        if node.is_none() || *node.unwrap() == self.empty {
            return;
        }
        self.traverse_postorder(BinaryTree::<T>::left_idx(k), f);
        self.traverse_postorder(BinaryTree::<T>::right_idx(k), f);
        f((k, node));
    }

    pub fn traverse_inorder<F>(&self, k: usize, f: &mut F)
    where
        F: FnMut((usize, Option<&T>)),
    {
        if k >= self.mem.len() {
            return;
        }
        let node = self.mem.get(k);
        if node.is_none() || *node.unwrap() == self.empty {
            return;
        }

        self.traverse_inorder(BinaryTree::<T>::left_idx(k), f);
        f((k, node));

        self.traverse_inorder(BinaryTree::<T>::right_idx(k), f);
    }
}
