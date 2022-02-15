use std::fmt::Debug;

// O(n^2)
pub fn bubble_sort_asc<T: PartialOrd + Debug>(v: &mut [T]) {
    println!("bubble-sort ascending");
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn bubble_sort_desc<T: PartialOrd + Debug>(v: &mut [T]) {
    println!("bubble-sort descending");
    for p in 0..v.len() {
        let mut sorted = true;
        // for i in 0..(v.len() - 1) - p {
        let mut i = v.len() - 1;
        while i > p {
            if v[i] < v[i - 1] {
                v.swap(i, i - 1);
                sorted = false;
            }
            i -= 1;
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half,
    // sort the right half, O(n * ln(n))
    // bring the sorted halfs together O(n)

    println!("MS: {:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together again; add whichever is lowest, the front of a
    // or the front of b
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

// Move first element to the correct place
// Everything lower should be before it,
// everything higher should be after it,
// return its location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("QS: {:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // Middle element already sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v1 = vec![7, 6, 3, 5, 4, 2, 1];
        let mut v2 = v1.clone();

        bubble_sort_asc(&mut v1);
        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6, 7]);

        bubble_sort_desc(&mut v2);
        assert_eq!(v2, vec![1, 2, 3, 4, 5, 6, 7]);

        let mut v1 = vec![4, 6, 1, 8, 11, 13, 3];
        let mut v2 = v1.clone();

        bubble_sort_asc(&mut v1);
        assert_eq!(v1, vec![1, 3, 4, 6, 8, 11, 13]);

        bubble_sort_desc(&mut v2);
        assert_eq!(v2, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_merge_sort() {
        let v1 = vec![4, 6, 1, 8, 11, 13, 3];
        let v1 = merge_sort(v1);
        assert_eq!(v1, vec![1, 3, 4, 6, 8, 11, 13]);

        let v1 = vec![7, 6, 3, 5, 4, 2, 1];
        let v1 = merge_sort(v1);
        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_pivot() {
        let mut v1 = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v1);

        for x in 0..v1.len() {
            assert!((v1[x] < v1[p]) == (x < p));
        }
        assert_eq!(2, p);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        println!("quick sort vector: {:?}", v);
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        let mut v = vec![7, 6, 3, 5, 4, 2, 1];
        println!("quick sort vector: {:?}", v);
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7]);

        let mut v = vec![1, 2, 3, 4, 5, 7, 6];
        println!("quick sort vector: {:?}", v);
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
