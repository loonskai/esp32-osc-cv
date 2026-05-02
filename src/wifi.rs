use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use esp_idf_hal::{
    modem::{Modem}
};
use anyhow::{Ok, Result};

pub fn connect_wifi(
    modem: Modem<'static>,
    sysloop: EspSystemEventLoop,
) -> Result<BlockingWifi<EspWifi<'static>>> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), None)?,
        sysloop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: env!("WIFI_SSID").try_into().unwrap(),
        password: env!("WIFI_PASSWORD").try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    let ip = wifi.wifi().sta_netif().get_ip_info()?;
    println!("WiFi connected - IP {:?}", ip.ip);

    Ok(wifi)
}
