/// Finds the number dividing a real number space into two parts using binary search up to a precision of epsilon.
/// Yields undefined behavior when the search space does not consist of exactly two areas.
/// The test function is expected to return true if the tested number is below the bound.
pub fn binary_search<F>(mut low: f64, mut high: f64, epsilon: f64, test: F) -> f64
where
    F: Fn(f64) -> bool,
{
    while high - low >= epsilon {
        let middle = (high + low) / 2.0;
        let below = test(middle);

        if below {
            low = middle;
        } else {
            high = middle;
        }
    }

    return (high + low) / 2.0;
}
