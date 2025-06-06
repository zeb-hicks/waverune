#[allow(unused)]
pub fn diff(a: Vec<u8>, b: Vec<u8>) -> Vec<(usize, Vec<u8>)> {
    let mut c = Vec::new();
    let diffs = Vec::new();

    c.resize(a.len().max(b.len()), None);

    let min = a.len().min(b.len());
    let max = a.len().max(b.len());
    for i in 0..min {
        c[i] = match a[i] == b[i] {
            true => Some(b[i]),
            false => None,
        };
    }
    for i in min..max {
        c[i] = match a.len() > b.len() {
            true => Some(0),
            false => Some(b[i]),
        };
    }

    let i = 0;
    while i < max {

    }

    diffs
}

#[test]
fn test_diff() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![1, 2, 6, 7, 8];
    let expected = vec![(2, vec![6, 7, 8])];
    assert_eq!(diff(a.clone(), b.clone()), expected);

    let a = vec![1, 2, 3, 4, 5, 6, 7,  8, 9, 10, 15, 16, 17, 18];
    let b = vec![1, 2, 3, 5, 6, 7, 15, 8, 9, 10, 11, 12, 17, 18, 19, 20, 22];
    let expected = vec![(3, vec![5, 6, 7, 15]), (10, vec![11, 12]), (14, vec![19, 20, 22])];
    assert_eq!(diff(a.clone(), b.clone()), expected);

    let a = vec![1, 2, 3, 4, 5];
    let b = vec![1, 2, 3];
    let expected = vec![(3, vec![0, 0])];
    assert_eq!(diff(a.clone(), b.clone()), expected);
}
