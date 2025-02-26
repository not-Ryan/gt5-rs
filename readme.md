# GT5-rs

Rust library for the GT-521F52, GT-521F32, and GT-511C3(R) fingerprint scanners.

It uses `nusb` under the hood.

## Features

- Support for the most of the [GT-521F52 API](https://cdn.sparkfun.com/assets/learn_tutorials/7/2/3/GT-521F52_Programming_guide_V10_20161001.pdf)
- SCSI-over-USB; no adapter required

## Usage

```rust
use gt5::autodiscover;

#[tokio::main]
async fn main() {
    let gt5 = autodiscover().await.expect("Could not get gt5");

    match gt5.get_enroll_count().await {
        Ok(count) => println!("Enrolled count: {count}"),
        Err(err) => eprintln!("Could not get enrolled count: {err}"),
    }
}
```

See `examples/` for more examples.

## Known issuess

- You are unable to run this on MacOSX, as you are unable to claim the USB device. This is a limitation of the nusb library.