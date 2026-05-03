use esp_idf_hal::{
    peripherals::{Peripherals},
    spi::{SpiDeviceDriver, SpiDriver, SpiDriverConfig, config::Config},
    units::{FromValueType}
};
use esp_idf_svc::{eventloop::EspSystemEventLoop};
use std::net::UdpSocket;
use rosc::{OscPacket, decoder};

mod wifi;
mod dac;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        esp_idf_svc::sys::nvs_flash_init();
    }

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    let (_wifi, _sysloop) = wifi::connect_wifi(peripherals.modem, sysloop.clone())?;
    
    let spi_driver = SpiDriver::new(
        peripherals.spi2, 
        peripherals.pins.gpio18, 
        peripherals.pins.gpio23, 
        None::<esp_idf_hal::gpio::AnyIOPin>, 
        &SpiDriverConfig::new(),
    )?;

    let spi_device = SpiDeviceDriver::new(
        spi_driver, 
        Some(peripherals.pins.gpio5), 
        &Config::new().baudrate(1_000_000u32.Hz()),
    )?;

    let mut dac = dac::Mcp4822::new(spi_device);

    let socket = UdpSocket::bind("0.0.0.0:8000")?;
    socket.set_nonblocking(false)?;
    println!("Listening on port 8000");

    let mut buf = [0u8; 256];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, _src)) => {
                match decoder::decode_udp(&buf[..len]) {
                    Ok((_, OscPacket::Message(msg))) => {
                        if let Some(rosc::OscType::Float(val)) = msg.args.first() {
                            let channel = if msg.addr == "/cv/1" { 0 } else { 1 };
                            dac.write_float(channel, *val);
                            println!("CV {} -> {}", channel, val);
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => {
                println!("recv error: {}", e);
                // keep looping, don't return
            }
        } 
    }

    // _wifi stays alive until here
    #[allow(unreachable_code)]
    drop(_wifi);
}
