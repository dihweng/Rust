/// # Cutting a Rod via Dynamic Programming

/// `rod_cut(prices)` returns the maximum price for the given length rod length
/// This function determines the maximum value obtainable by cutting up the rod
/// and selling the pieces.
///
/// Assumptions: prices.len() > 0
pub fn rod_cut(prices: Vec<u32>) -> u32 {
    let n = prices.len();

    assert!(n > 0);

    let mut values: Vec<u32> = vec![0];

    for i in 1..=n {
        let mut max_value = std::u32::MIN;

        // incrementally search for the best price up until i
        for j in 0..i {
            // take max price, which is either the current price or
            // the current price plus the last max price
            max_value = std::cmp::max(
                max_value,
                prices.get(j).unwrap() + values.get(i - j - 1).unwrap(),
            );
        }

        // place the best price up in i (last position)
        values.insert(i, max_value);
    }

    if let Some(last_element) = values.last() {
        return *last_element;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::rod_cut;

    #[test]
    fn results_22() {
        assert_eq!(rod_cut(vec![1, 5, 8, 9, 10, 17, 17, 20],), 22);
    }

    #[test]
    fn results_24() {
        assert_eq!(rod_cut(vec![3, 5, 8, 9, 10, 17, 17, 20]), 24);
    }

    #[test]
    fn results_12() {
        assert_eq!(rod_cut(vec![6, 1]), 12);
    }

    #[test]
    #[should_panic]
    fn results_panic() {
        assert_eq!(rod_cut(vec![]), 24);
    }

    #[test]
    fn results_0() {
        assert_eq!(rod_cut(vec![0, 0, 0, 0, 0]), 0);
    }
}
