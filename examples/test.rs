use zelkova::{models::*, tsr};

fn main() {
    let tensor = tsr![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12],];

    println!("{}", tensor.order);
}
