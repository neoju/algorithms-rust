use rand::random;

pub fn sort() {
    let mut arr: [u8; 10] = random();
    let len = arr.len();
    merge_sort(&mut arr, 0, len - 1);
    println!("merge_sort - {}", arr.is_sorted());
}

fn merge_sort(arr: &mut [u8; 10], l: usize, r: usize) {
    if l < r {
        let m = l + (r - l) / 2;
        merge_sort(arr, l, m);
        merge_sort(arr, m + 1, r);
        merge_parts(arr, l, m, r);
    }
}

fn merge_parts(arr: &mut [u8; 10], l: usize, m: usize, r: usize) {
    let mut left = arr[l..=m].to_vec();
    let mut right = arr[(m + 1)..=r].to_vec();
    let mut k = l;

    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            arr[k] = left.remove(0);
        } else {
            arr[k] = right.remove(0);
        }
        k += 1;
    }

    while !left.is_empty() {
        arr[k] = left.remove(0);
        k += 1;
    }

    while !right.is_empty() {
        arr[k] = right.remove(0);
        k += 1;
    }
}
