use embassy_time::{Duration, Timer};
use esp_hal::{
    gpio::Level,
    peripherals::{GPIO8, RMT},
    rmt::{PulseCode, Rmt, TxChannelConfig, TxChannelCreator as _},
    time::Rate,
};

const LED_PULSES: usize = 24;
const RMT_BUFFER_LEN: usize = LED_PULSES + 1;

const T0H_NS: u32 = 400;
const T0L_NS: u32 = 850;
const T1H_NS: u32 = 850;
const T1L_NS: u32 = 400;

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

const COLORS: [Rgb; 7] = [
    Rgb { r: 32, g: 0, b: 0 },
    Rgb { r: 32, g: 12, b: 0 },
    Rgb { r: 28, g: 24, b: 0 },
    Rgb { r: 0, g: 32, b: 0 },
    Rgb { r: 0, g: 24, b: 24 },
    Rgb { r: 0, g: 0, b: 32 },
    Rgb { r: 20, g: 0, b: 32 },
];

#[embassy_executor::task]
pub async fn blink(rmt: RMT<'static>, pin: GPIO8<'static>) {
    let rmt = Rmt::new(rmt, Rate::from_mhz(80)).expect("failed to initialize RMT");
    let tx_config = TxChannelConfig::default()
        .with_clk_divider(1)
        .with_idle_output_level(Level::Low)
        .with_carrier_modulation(false)
        .with_idle_output(true);
    let mut channel = rmt
        .channel0
        .configure_tx(&tx_config)
        .expect("failed to configure RMT TX")
        .with_pin(pin);
    let mut rmt_buffer = [PulseCode::end_marker(); RMT_BUFFER_LEN];
    let mut color_index = 0;

    loop {
        let color = COLORS[color_index];

        encode_rgb(color, &mut rmt_buffer);
        match channel.transmit(&rmt_buffer) {
            Ok(transaction) => match transaction.wait() {
                Ok(next_channel) => channel = next_channel,
                Err((_error, next_channel)) => channel = next_channel,
            },
            Err((_error, next_channel)) => {
                channel = next_channel;
            }
        }

        color_index = (color_index + 1) % COLORS.len();
        Timer::after(Duration::from_millis(250)).await;
    }
}

fn encode_rgb(color: Rgb, buffer: &mut [PulseCode; RMT_BUFFER_LEN]) {
    let zero = pulse(T0H_NS, T0L_NS);
    let one = pulse(T1H_NS, T1L_NS);
    let mut index = 0;

    // Addressable RGB LEDs usually expect GRB byte order on the wire
    for byte in [color.g, color.r, color.b] {
        for bit in (0..8).rev() {
            buffer[index] = if byte & (1 << bit) == 0 { zero } else { one };
            index += 1;
        }
    }

    buffer[index] = PulseCode::end_marker();
}

fn pulse(high_ns: u32, low_ns: u32) -> PulseCode {
    PulseCode::new(Level::High, ticks(high_ns), Level::Low, ticks(low_ns))
}

fn ticks(ns: u32) -> u16 {
    ((ns * 80) / 1000) as u16
}
