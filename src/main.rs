mod sort;

fn main() {
    sort::basic::bubble_sort();
    sort::basic::insertion_sort();
    sort::basic::selection_sort();

    sort::merge_sort::sort();
}
