use ff_cosmwasm_std::count;

pub fn count_all<T>(matrix: Vec<Vec<T>>) -> Vec<usize> {
    matrix.iter().map(|row| count(row)).collect()
}
