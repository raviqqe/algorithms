pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let i = nums1.len() / 2;
    let j = nums2.len() / 2;

    while i + j == (nums1.len() + nums2.len()) / 2 {
        todo!();
    }

    nums1[i] as f64 + nums2[j] as f64
}
