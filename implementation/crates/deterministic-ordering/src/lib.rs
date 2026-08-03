pub fn stable_sort_by_key<T, K: Ord, F: FnMut(&T) -> K>(items: &mut [T], mut key: F) {
    items.sort_by_key(|item| key(item));
}
pub fn compare_keys(left: &str, right: &str) -> std::cmp::Ordering {
    left.as_bytes().cmp(right.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_does_not_depend_on_input_order() {
        let mut a = vec!["z", "a", "m"];
        stable_sort_by_key(&mut a, |x| *x);
        assert_eq!(a, vec!["a", "m", "z"]);
    }
}
