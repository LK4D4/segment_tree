mod segment_tree;
pub use crate::segment_tree::SegmentTree;

#[cfg(test)]
mod tests {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    use super::*;
    #[test]
    fn range_op() {
        let st = SegmentTree::new(vec![1, 2, 3], add, 0);
        assert_eq!(st.range_op(0, 1), 1, "[0, 1)");
        assert_eq!(st.range_op(1, 2), 2, "[1, 2)");
        assert_eq!(st.range_op(2, 3), 3, "[2, 3)");

        assert_eq!(st.range_op(0, 2), 3, "[0, 2)");
        assert_eq!(st.range_op(0, 3), 6, "[0, 3)");
    }

    #[test]
    fn replace() {
        let mut st = SegmentTree::new(vec![1, 2, 3], add, 0);
        st.replace(1, 5);
        assert_eq!(st.range_op(0, 2), 6, "[0, 2)");
        assert_eq!(st.range_op(0, 3), 9, "[0, 3)");
    }

    #[test]
    fn single_node() {
        let st = SegmentTree::new(vec![1], add, 0);
        assert_eq!(st.range_op(0, 2), 1, "[0, 2)");
    }

    #[test]
    fn min_tree() {
        let mut st = SegmentTree::new(vec![1, 2, 3], std::cmp::min, std::i32::MAX);
        assert_eq!(st.range_op(0, 2), 1, "min [0, 2)");
        assert_eq!(st.range_op(1, 3), 2, "min [1, 3)");
        st.replace(0, 10);
        assert_eq!(st.range_op(0, 2), 2, "min [0, 2)");
    }
}
