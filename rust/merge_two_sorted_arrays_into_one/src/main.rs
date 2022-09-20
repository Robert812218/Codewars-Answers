use itertools::Itertools;

fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    new_vec.extend(arr1);
    new_vec.extend(arr2);
    new_vec.sort();
    new_vec.dedup();

    new_vec
}
