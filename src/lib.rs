mod segment_tree;
pub use crate::segment_tree::SegmentTree;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_sum() {
        let st = SegmentTree::new(vec![1, 2, 3]);
        assert_eq!(st.range_sum(0, 1), 1, "[0, 1)");
        assert_eq!(st.range_sum(1, 2), 2, "[1, 2)");
        assert_eq!(st.range_sum(2, 3), 3, "[2, 3)");

        assert_eq!(st.range_sum(0, 2), 3, "[0, 2)");
        assert_eq!(st.range_sum(0, 3), 6, "[0, 3)");
    }

    #[test]
    fn replace() {
        let mut st = SegmentTree::new(vec![1, 2, 3]);
        st.replace(1, 5);
        assert_eq!(st.range_sum(0, 2), 6, "[0, 2)");
        assert_eq!(st.range_sum(0, 3), 9, "[0, 3)");
    }
}
