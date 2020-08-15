use std::ops::Add;

pub struct SegmentTree<T> {
    data: Vec<T>,
}

impl<T: Default + Add<Output = T> + Copy> SegmentTree<T> {
    fn segmentify(data: &mut Vec<T>) {
        for i in (1..=data.len() - 2).rev().step_by(2) {
            let parent_idx = i / 2;
            data[parent_idx] = data[i] + data[i + 1];
        }
    }

    pub fn new(mut data: Vec<T>) -> Self {
        let internal_len = data.len().checked_next_power_of_two().unwrap();
        data.resize_with(internal_len, Default::default);
        let mut internal_data = Vec::with_capacity(internal_len * 2 - 1);
        internal_data.resize_with(internal_len - 1, Default::default);
        internal_data.extend(data);
        SegmentTree::segmentify(&mut internal_data);
        SegmentTree {
            data: internal_data,
        }
    }

    fn get_in_tree_idx(&self, idx: usize) -> usize {
        self.data.len() / 2 + idx
    }
    pub fn replace(&mut self, idx: usize, val: T) {
        let mut in_tree_idx = self.get_in_tree_idx(idx);
        self.data[in_tree_idx] = val;
        while in_tree_idx > 0 {
            let parent_idx = (in_tree_idx - 1) / 2;
            self.data[parent_idx] = self.data[parent_idx * 2 + 1] + self.data[parent_idx * 2 + 2];
            in_tree_idx = parent_idx;
        }
    }
    fn range_sum_helper(&self, node_idx: usize, l: usize, r: usize, lx: usize, rx: usize) -> T {
        if l <= lx && r >= rx {
            self.data[node_idx]
        } else if rx <= l || lx >= r {
            T::default()
        } else {
            let mid = (lx + rx) / 2;
            self.range_sum_helper(2 * node_idx + 1, l, r, lx, mid)
                + self.range_sum_helper(2 * node_idx + 2, l, r, mid, rx)
        }
    }
    pub fn range_sum(&self, l: usize, r: usize) -> T {
        assert!(r < self.data.len());
        assert!(l != r);
        self.range_sum_helper(
            0,
            self.get_in_tree_idx(l),
            self.get_in_tree_idx(r),
            self.data.len() / 2,
            self.data.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_replace() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut st = SegmentTree::new(v);
        assert_eq!(
            st.data,
            vec![21, 10, 11, 3, 7, 11, 0, 1, 2, 3, 4, 5, 6, 0, 0]
        );
        st.replace(3, 7); // 4 -> 7
        assert_eq!(
            st.data,
            vec![24, 13, 11, 3, 10, 11, 0, 1, 2, 3, 7, 5, 6, 0, 0]
        );
    }
}
