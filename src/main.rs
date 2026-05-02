use esp_idf_hal::peripherals::{Peripherals};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};

mod wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    let _wifi = wifi::connect_wifi(peripherals.modem, sysloop)?;

    // Holds WiFi alive - drop this and connection dies
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
