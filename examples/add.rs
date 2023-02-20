use zelkova::Device;

fn main() {
    let mut device = Device::default();

    let mut layer0 = [0.0_f32, 0.0];
    let mut layer1 = [0.0_f32, 0.0, 0.0];
    let mut layer2 = [0.0_f32];

    device.bind("layer0", &mut layer0);
    device.bind("layer1", &mut layer1);
    device.bind("layer2", &mut layer2);

    println!("{device:?}");

    device.compute(
        r#"
        let len = arrayLength(&layer0);

        if (index.x >= len) {
            return;
        }

        layer0[index.x] += layer1[index.y];
    "#,
        [2, 3, 1],
    );
}
