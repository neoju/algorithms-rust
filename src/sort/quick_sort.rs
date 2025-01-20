use rand::random;

pub fn sort() {
    let mut arr: [u8; 10] = random();
    let len = arr.len();
    quick_sort(&mut arr, 0, len - 1);
    println!("quick_sort - {}", arr.is_sorted());
}

fn quick_sort(arr: &mut [u8; 10], l: usize, r: usize) {
    if l < r {
        let pi = partition(arr, l, r);
        // saturating_sub is used to prevent overflow subtraction
        quick_sort(arr, l, pi.saturating_sub(1));
        quick_sort(arr, pi + 1, r);
    }
}

fn partition(arr: &mut [u8; 10], l: usize, r: usize) -> usize {
    let pivot = arr[r];
    let mut i = l;

    for j in l..r {
        if arr[j] < pivot {
            if i != j {
                arr.swap(i, j);
            }

            i += 1;
        }
    }

    arr.swap(i, r);
    i
}
