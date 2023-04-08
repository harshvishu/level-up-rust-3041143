fn main() {
    let mut list: Vec<f32> = vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 5.0, 6.0];
    hello_rust_learners::unique(&mut list);
    print!("the list after removing duplicates is: {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        let mut list: Vec<f32> = vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 5.0, 6.0];
        hello_rust_learners::unique(&mut list);
        assert_eq!(list, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }
}

/*
fn median(list: &mut [f32]) -> Option<f32> {
    if list.is_empty() {
        return None;
    }

    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid_index = list.len() / 2;

    if list.len() % 2 == 0 {
        let mid1 = list[mid_index];
        let mid2 = list[mid_index - 1];

        Some((mid1 + mid2) / 2.0)
    } else {
        Some(list[mid_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        let mut list: Vec<f32> = vec![1.0, 4.0, 5.0];
        assert_eq!(median(&mut list), Some(4.0));
    }

    #[test]
    fn test_median_even() {
        let mut list: Vec<f32> = vec![3.0, 1.5, 8.8, 5.0];
        assert_eq!(median(&mut list), Some(4.0));
    }

    #[test]
    fn test_median_empty() {
        let mut list: Vec<f32> = vec![];
        assert_eq!(median(&mut list), None);
    }
}
*/
