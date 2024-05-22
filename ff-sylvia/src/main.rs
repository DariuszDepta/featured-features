use ff_multitest::count_all;

pub fn main() {
    let data = vec![vec![1, 2, 3], vec![1, 2], vec![27, 15, 33, 1]];
    println!("{:?}", count_all(data));
}
