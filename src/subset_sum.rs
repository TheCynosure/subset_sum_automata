pub fn check_no_empty_subset(vec: Vec<i32>, target_sum: i32) -> bool {
    let mut subset_mask = 1;
    let subset_limit =  2_i32.pow(vec.len() as u32);
    while subset_mask < subset_limit {
        let mut mask_copy = subset_mask;
        let mut sum: i32 = 0;
        for (i, _) in vec.iter().enumerate() {
            if (mask_copy & 1) == 1 {
                sum += vec[i];
            }
            mask_copy = mask_copy >> 1;    
        }

        if sum == target_sum {
            return true;     
        }
        subset_mask += 1;
    }
    false
}