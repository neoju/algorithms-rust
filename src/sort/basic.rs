use rand::random;

pub fn bubble_sort() {
    let mut arr: [u8; 10] = random();
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(i, j);
            }
        }
    }
    println!("bubble_sort - {}", arr.is_sorted());
}

pub fn insertion_sort() {
    let mut arr: [u8; 10] = random();
    for i in 1..arr.len() {
        let mut prev = i - 1;
        while arr[prev + 1] < arr[prev] {
            arr.swap(prev + 1, prev);

            if prev == 0 {
                break;
            }
            prev -= 1;
        }
    }
    println!("insertion_sort - {}", arr.is_sorted());
}

pub fn selection_sort() {
    let mut arr: [u8; 10] = random();
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
    println!("selection_sort - {}", arr.is_sorted());
}
