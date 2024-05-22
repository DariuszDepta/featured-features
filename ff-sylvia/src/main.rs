pub fn main() {
    let data = vec![vec![1, 2, 3], vec![1, 2], vec![27, 15, 33, 1]];
    println!("{:?}", ff_multitest::count_all(data));

    ff_cosmwasm_std::count(&[1, 2, 3]);
}
