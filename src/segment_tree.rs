use std::ops::Add;

type Binop<T> = fn(T, T) -> T;

pub struct SegmentTree<T> {
    data: Vec<T>,
    op: Binop<T>,
}

impl<T: Default + Add<Output = T> + Copy> SegmentTree<T> {
    fn segmentify(&mut self) {
        for i in (2..self.data.len()).rev().step_by(2) {
            let parent_idx = (i - 1) / 2;
            self.data[parent_idx] = (self.op)(self.data[i - 1], self.data[i]);
        }
    }

    pub fn new(mut data: Vec<T>, op: Binop<T>) -> Self {
        let internal_len = data.len().checked_next_power_of_two().unwrap();
        data.resize_with(internal_len, Default::default);
        let mut internal_data = Vec::with_capacity(internal_len * 2 - 1);
        internal_data.resize_with(internal_len - 1, Default::default);
        internal_data.extend(data);
        let mut st = SegmentTree {
            data: internal_data,
            op,
        };
        st.segmentify();
        st
    }

    fn get_in_tree_idx(&self, idx: usize) -> usize {
        self.data.len() / 2 + idx
    }
    pub fn replace(&mut self, idx: usize, val: T) {
        let mut in_tree_idx = self.get_in_tree_idx(idx);
        self.data[in_tree_idx] = val;
        while in_tree_idx > 0 {
            let parent_idx = (in_tree_idx - 1) / 2;
            self.data[parent_idx] =
                (self.op)(self.data[parent_idx * 2 + 1], self.data[parent_idx * 2 + 2]);
            in_tree_idx = parent_idx;
        }
    }
    fn range_op_helper(&self, node_idx: usize, l: usize, r: usize, lx: usize, rx: usize) -> T {
        if l <= lx && r >= rx {
            self.data[node_idx]
        } else if rx <= l || lx >= r {
            T::default()
        } else {
            let mid = (lx + rx) / 2;
            (self.op)(
                self.range_op_helper(2 * node_idx + 1, l, r, lx, mid),
                self.range_op_helper(2 * node_idx + 2, l, r, mid, rx),
            )
        }
    }
    pub fn range_op(&self, l: usize, r: usize) -> T {
        let r = if r > self.data.len() {
            self.data.len()
        } else {
            r
        };
        assert!(l != r);
        self.range_op_helper(
            0,
            self.get_in_tree_idx(l),
            self.get_in_tree_idx(r),
            self.data.len() / 2,
            self.data.len(),
        )
    }
}
