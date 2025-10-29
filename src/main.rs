use hidapi::HidApi;
use std::error::Error;
use std::io::{self, Write};

fn format_option<T: std::fmt::Debug>(opt: Option<T>) -> String {
    match opt {
        Some(val) => format!("{val:?}"),
        None => "N/A".to_string(),
    }
}

fn format_raw_option(opt: Option<&[u16]>) -> String {
    match opt {
        Some(val) => {
            let raw_vals = format!("{val:?}");
            let utf16_str = String::from_utf16_lossy(val);
            format!("| Raw: [Raw: {raw_vals}, Decoded: {utf16_str}]")
        }
        None => String::new(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;

    println!("Connected USB HID Devices:");
    println!("-------------------------");

    for device in api.device_list() {
        println!("Device path: {}", device.path().to_string_lossy());

        println!(
            "VID: 0x{:04X} | PID: 0x{:04X} | Serial: {}{}",
            device.vendor_id(),
            device.product_id(),
            format_option(device.serial_number()),
            format_raw_option(device.serial_number_raw())
        );

        println!(
            "Manufacturer: {}{} | Product: {}{}",
            format_option(device.manufacturer_string()),
            format_raw_option(device.manufacturer_string_raw()),
            format_option(device.product_string()),
            format_raw_option(device.product_string_raw())
        );

        #[cfg(target_os = "linux")]
        println!(
            "Release: 0x{:04X} | Interface: {} | Bus: {:?} | Usage Page/Usage: N/A (Linux)",
            device.release_number(),
            device.interface_number(),
            device.bus_type()
        );

        #[cfg(not(target_os = "linux"))]
        println!(
            "Release: 0x{:04X} | Interface: {} | Bus: {:?} | Usage Page: {} | Usage: {}",
            device.release_number(),
            device.interface_number(),
            device.bus_type(),
            device.usage_page(),
            device.usage()
        );

        println!("-------------------------");
    }

    print!("\nPress Enter to exit...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
