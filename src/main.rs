use brightness::Brightness;
use chrono::{Local, Timelike};
use futures::{executor::block_on, TryStreamExt};
use std::{thread, time::Duration};

async fn set_brightness(val: u32) -> Result<(), brightness::Error> {
    brightness::brightness_devices()
        .try_for_each(|mut dev| async move {
            let name = dev.device_name().await?;
            dev.set(val).await?;
            println!("Brightness of device {} is set to {}%", name, val);
            Ok(())
        })
        .await
}

fn compute_brightness() -> u32 {
    let now = Local::now();
    let hour = now.hour();

    return match hour {
        6..=9 => 80,
        10..=15 => 100,
        16..=17 => 60,
        18..=19 => 50,
        20..=22 => 30,
        23 => 20,
        _ => 0,
    };
}

fn main() {
    let mut prev_brightness: Option<u32> = None;

    loop {
        let brightness = compute_brightness();

        if prev_brightness == Some(brightness) {
            continue;
        }

        block_on(set_brightness(brightness)).unwrap();

        prev_brightness = Some(brightness);
        thread::sleep(Duration::from_secs(10));
    }
}
