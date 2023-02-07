use std::array;
use zelkova::Device;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let device = Device::new().await.unwrap();

    let lhs: [f32; 512] = array::from_fn(|i| i as f32);
    let rhs: [f32; 512] = array::from_fn(|i| i as f32);

    let output = device.add(&lhs, &rhs).await;

    println!("{output:?}");
}
