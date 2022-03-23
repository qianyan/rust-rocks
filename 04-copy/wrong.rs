fn main() {
    let data = vec![1, 2, 3, 4];
    let dataOwned = data;
    println!("sum of dataOwned: {}", sum(dataOwned));
    println!("dataOwned: {:?}", dataOwned);
    println!("sum of data: {}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}
