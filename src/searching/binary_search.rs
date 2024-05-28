use std::cmp::Ordering;

/// Marker type for ascending order.
pub struct Ascending;

/// Marker type for descending order.
pub struct Descending;

#[derive(Debug, PartialEq, Eq)]
pub struct SortedArray<T: Ord, O> {
    data: Vec<T>,
    _order: std::marker::PhantomData<O>,
}

impl<T: Ord, O> SortedArray<T, O> {
    /// Returns a reference to the inner data slice.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns the length of the sorted array.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Ord> SortedArray<T, Ascending> {
    /// Creates a new SortedArray from a Vec in ascending order.
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort();
        Self {
            data,
            _order: std::marker::PhantomData,
        }
    }
}

impl<T: Ord> SortedArray<T, Descending> {
    /// Creates a new SortedArray from a Vec in descending order.
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort_by(|a, b| b.cmp(a));
        Self {
            data,
            _order: std::marker::PhantomData,
        }
    }
}

/// Performs a binary search on a sorted array to find all occurrences of the
/// specified item. This function searches for the item in the provided sorted
/// array and returns a vector containing the indices of all occurrences of the
/// item. The array is expected to be sorted according to the provided order.
pub fn binary_search<T: Ord, O>(item: &T, arr: &SortedArray<T, O>) -> Vec<usize>
where
    O: OrdOrder,
{
    let mut left = 0;
    let mut right = arr.len();
    let mut result = Vec::new();

    while left < right {
        let mid = left + (right - left) / 2;
        match O::cmp(item, &arr.as_slice()[mid]) {
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
            Ordering::Equal => {
                result = find_first_and_last::<T, O>(item, arr);
                break;
            }
        }
    }

    result
}

/// Finds the boundary (first or last occurrence) of an item in a sorted array.
fn find_boundary<T: Ord, O>(item: &T, arr: &SortedArray<T, O>, find_first: bool) -> usize
where
    O: OrdOrder,
{
    let mut left = 0;
    let mut right = arr.len();
    let mut boundary = 0;

    while left < right {
        let mid = left + (right - left) / 2;
        match (O::cmp(&arr.as_slice()[mid], item), find_first) {
            (Ordering::Equal, true) => {
                boundary = mid;
                right = mid;
            }
            (Ordering::Equal, false) => {
                boundary = mid;
                left = mid + 1;
            }
            (Ordering::Less, _) => left = mid + 1,
            (Ordering::Greater, _) => right = mid,
        }
    }

    boundary
}

/// Finds the first and last occurrences of an item in a sorted array starting from a given index.
fn find_first_and_last<T: Ord, O>(item: &T, arr: &SortedArray<T, O>) -> Vec<usize>
where
    O: OrdOrder,
{
    let first = find_boundary::<T, O>(item, arr, true);
    let last = find_boundary::<T, O>(item, arr, false);

    (first..=last).collect()
}

/// Trait to define the comparison behavior based on the ordering type.
pub trait OrdOrder {
    fn cmp<T: Ord>(a: &T, b: &T) -> Ordering;
}

impl OrdOrder for Ascending {
    fn cmp<T: Ord>(a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

impl OrdOrder for Descending {
    fn cmp<T: Ord>(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STR_ARR: &[&str] = &["b", "d", "c", "a", "zoo", "google"];
    const INT_ARR: &[i32] = &[3, 2, 4, 1];
    const INT_ARR_WITH_GAP: &[i32] = &[11, 3, 8, 1];
    const DUPLICATE_ARR: &[i32] = &[1, 2, 5, 3, 2, 3, 3, 4, 4, 5];
    const LARGE_ARR: &[i32] = &[
        5, 1, 3, 4, 2, 3, 7, 6, 3, 9, 3, 3, 8, 3, 3, 2, 0, 3, 3, 10, 3, 3, 3,
    ];

    macro_rules! test_binary_search {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, data, expected_asc, expected_desc) = $test_case;

                    let sorted_array = SortedArray::<_, Ascending>::new(data.to_vec());
                    assert_eq!(binary_search(&item, &sorted_array), expected_asc);

                    let sorted_array = SortedArray::<_, Descending>::new(data.to_vec());
                    assert_eq!(binary_search(&item, &sorted_array), expected_desc);
                }
            )*
        };
    }

    test_binary_search! {
        one_item_found_str: ("a", ["a"], vec![0], vec![0]),
        one_item_not_found_str: ("b", ["a"], Vec::new(), Vec::new()),
        one_item_found_int: (1, [1], vec![0], vec![0]),
        one_item_not_found_int: (2, [1], Vec::new(), Vec::new()),
        empty_str: ("a", &[] as &[&str], Vec::new(), Vec::new()),
        empty_int: (1, &[] as &[i32], Vec::new(), Vec::new()),
        search_strings_start: ("a", STR_ARR, vec![0], vec![5]),
        search_strings_middle: ("google", STR_ARR, vec![4], vec![1]),
        search_strings_last: ("zoo", STR_ARR, vec![5], vec![0]),
        search_strings_not_found: ("x", STR_ARR, Vec::new(), Vec::new()),
        search_ints_start: (1, INT_ARR, vec![0], vec![3]),
        search_ints_middle: (3, INT_ARR, vec![2], vec![1]),
        search_ints_end: (4, INT_ARR, vec![3], vec![0]),
        search_ints_not_found: (5, INT_ARR, Vec::new(), Vec::new()),
        with_gaps_0: (0, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_1: (1, INT_ARR_WITH_GAP, vec![0], vec![3]),
        with_gaps_2: (2, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_3: (3, INT_ARR_WITH_GAP, vec![1], vec![2]),
        with_gaps_4: (4, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_5: (5, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_6: (6, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_7: (7, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_8: (8, INT_ARR_WITH_GAP, vec![2], vec![1]),
        with_gaps_9: (9, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_10: (10, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_11: (11, INT_ARR_WITH_GAP, vec![3], vec![0]),
        with_gaps_12: (12, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        with_gaps_13: (13, INT_ARR_WITH_GAP, Vec::new(), Vec::new()),
        search_duplicates_first: (2, DUPLICATE_ARR, vec![1, 2], vec![7, 8]),
        search_duplicates_middle: (3, DUPLICATE_ARR, vec![3, 4, 5], vec![4, 5, 6]),
        search_duplicates_end: (5, DUPLICATE_ARR, vec![8, 9], vec![0, 1]),
        search_duplicates_not_found: (6, DUPLICATE_ARR, Vec::new(), Vec::new()),
        search_on_large_arr: (
            3,
            LARGE_ARR,
            vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
        ),
    }
}
