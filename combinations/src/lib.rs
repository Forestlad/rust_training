#![forbid(unsafe_code)]

fn factorial(n: usize) -> usize
{
    if (n == 0) || (n == 1) {
        return 1;
    }
    (2..=n).product()
}

fn number_combinations(n: usize, mut k: usize) -> usize
{
    if (k == 0) || (k == n) {
        return 1;
    }
    k = k.min(n - k);
    let res: usize = ((n - k + 1)..=n).product();
    res / factorial(k)
}

fn make_permutation(n: usize, arr: &[i32], perm: &mut Vec<i32>, res: &mut Vec<Vec<i32>>)
{
    if n == 0 {
        res.push(perm.clone());
        return;
    }
    if n == 1 {
        let s = perm.len();
        for i in 0..arr.len() {
            perm[s - 1] = arr[i];
            res.push(perm.clone());
        }
        return;
    }
    let s = perm.len();
    for i in 0..(arr.len() - n + 1) {
        perm[s - n] = arr[i];
        make_permutation(n - 1, &arr[(i + 1)..], perm, res);
    }

}

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>>
{
    if arr.len() < k {
        return Vec::<Vec<i32>>::new();
    }
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(number_combinations(arr.len(), k));
    let mut perm = vec![0; k];
    make_permutation(k, arr, &mut perm, &mut res);
    res
}
