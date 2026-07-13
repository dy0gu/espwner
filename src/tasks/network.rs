use alloc::string::String;
use defmt::{info, warn};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::WIFI;
use esp_radio::wifi::{self, scan::ScanConfig};

const SCAN_INTERVAL: Duration = Duration::from_secs(12);

#[embassy_executor::task]
pub async fn scan(wifi: WIFI<'static>) {
    let (mut controller, _) =
        wifi::new(wifi, Default::default()).expect("failed to initialize Wi-Fi");

    loop {
        match controller.scan_async(&ScanConfig::default()).await {
            Ok(mut networks) => {
                networks.sort_by(|left, right| right.signal_strength.cmp(&left.signal_strength));

                let mut ssids = String::new();
                for (_, network) in networks.iter().enumerate() {
                    ssids.push_str("\n\t");
                    ssids.push_str(network.ssid.as_str());
                    ssids.push(' ');
                    ssids.push_str(signal_bars(network.signal_strength));
                }
                info!(
                    "Scanned {} Wi-Fi networks: {}",
                    networks.len(),
                    ssids.as_str()
                );
            }
            Err(error) => warn!("Wi-Fi scan failed: {:?}", error),
        }

        Timer::after(SCAN_INTERVAL).await;
    }
}

fn signal_bars(rssi: i8) -> &'static str {
    match rssi {
        -70..=0 => "▁▅█",
        -90..=-71 => "▁▅",
        _ => "▁",
    }
}
