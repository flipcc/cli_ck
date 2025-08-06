use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find output device");

    println!("Output device: {}", device.name().unwrap());
}
