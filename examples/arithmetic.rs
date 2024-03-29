use zelkova::{self, tsr, Tensor, TensorOrder};

#[allow(unused)]
fn main() {
    let instance = zelkova::init();

    let t1 = tsr![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12],];
    let t2 = tsr![[2, 4, 6, 8], [10, 12, 14, 16], [18, 20, 22, 24]];

    let t3 = t1 + t2;

    // instance.resolve(t3); for actually running the chained operations.

    println!("{}", t3.order);
}
