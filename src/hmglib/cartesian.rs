// Copyright (c) 2022 Blacknon. All rights reserved.
// Reference:
//   - https://gist.github.com/kylewlacy/115965b40e02a3325558

/// Given a vector containing a partial Cartesian product, and a list of items,
/// return a vector adding the list of items to the partial Cartesian product.
///
/// # Example
///
/// ```
/// let partial_product = vec![vec![1, 4], vec![1, 5], vec![2, 4], vec![2, 5]];
/// let items = &[6, 7];
/// let next_product = partial_cartesian(partial_product, items);
/// assert_eq!(next_product, vec![vec![1, 4, 6],
///                               vec![1, 4, 7],
///                               vec![1, 5, 6],
///                               vec![1, 5, 7],
///                               vec![2, 4, 6],
///                               vec![2, 4, 7],
///                               vec![2, 5, 6],
///                               vec![2, 5, 7]]);
/// ```
///
fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
    a.into_iter()
        .flat_map(|xs| {
            b.iter()
                .cloned()
                .map(|y| {
                    let mut vec = xs.clone();
                    vec.push(y);
                    vec
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Computes the Cartesian product of lists[0] * lists[1] * ... * lists[n].
///
/// # Example
///
/// ```
/// let lists: &[&[_]] = &[&["a"], &["a", "b", "c"], &["a", "b", "c"]];
/// let product = cartesian_product(lists);
/// assert_eq!(product, vec![vec!["a","a","a"],
///                          vec!["a","a","b"],
///                          vec!["a","a","c"],
///                          vec!["a","b","a"],
///                          vec!["a","b","b"],
///                          vec!["a","b","c"],
///                          vec!["a","c","a"],
///                          vec!["a","c","b"],
///                          vec!["a","c","c"]]);
/// ```
fn cartesian_product<T: Clone>(lists: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter()
                .cloned()
                .fold(init, |vec, list| partial_cartesian(vec, &list[..]))
        }
        None => {
            vec![]
        }
    }
}

/// Print the Cartesian product of a set of lists to stdout, in
/// the following form:
///
/// ```text
/// aaa
/// aab
/// aac
/// aba
/// abc
/// abd
/// ...
/// ```
pub fn get_cartesian_product(lists: &Vec<Vec<String>>) -> Vec<String> {
    let mut result = vec![];
    let products = cartesian_product(lists);

    for product in products.iter() {
        let product_str: Vec<_> = product.iter().map(|n| format!("{}", n)).collect();
        let line = product_str.join("");
        result.push(line);
    }

    return result;
}
