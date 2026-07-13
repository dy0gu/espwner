# ESPwner 🔑

This project is intended only for authorized security testing, research, and educational purposes. Do not use it against networks, systems, or accounts without the explicit permission of their owner. You are solely responsible for complying with all applicable laws and for any misuse or resulting harm.

Some useful notes for developers:

- We do not have to explicitly define a `memory.x` file for this `probe-rs` project, as the `esp-hal` crate already provides a default linker script for the ESP-C6.

- The [.vscode/settings.json](./.vscode/settings.json) file should stay version controlled, otherwise `rust-analyzer` (by default) will constantly attempt to compile the project for the host
architecture, raising false errors. Other IDEs may also require similar configuration.

## Development 🛠️

1. Install the embedded toolkit via cargo: `cargo install probe-rs-tools --locked`

    - Linux users **must** also follow the [UDEV rules configuration](https://probe.rs/docs/getting-started/probe-setup/) step.

2. Install the espressif toolchain: `cargo install espflash --locked`

3. Connect the ESP-C6 via USB.

4. Use `cargo run` to flash the dev firmware to the microcontroller and initiate the serial connection for
debugging.

    - If the toolkit fails to find a probe, ensure your OS does not have any additional requirements mentioned in the [probe setup](https://probe.rs/docs/getting-started/probe-setup/) docs.

## Release 🚀

- Run `cargo build --release` and check the `target` folder for the compiled output.

- Alternatively, you can use `cargo run --release` to build and immediately flash the release firmware to the microcontroller.
