use hidapi::HidApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    
    println!("Enumerating all HID devices for Magi96 keyboard (VID: 0x320F, PID: 0x5088):\n");
    
    let devices: Vec<_> = api.device_list()
        .filter(|d| d.vendor_id() == 0x320F && d.product_id() == 0x5088)
        .collect();
    
    println!("Found {} HID interface(s):\n", devices.len());
    
    for (i, device_info) in devices.iter().enumerate() {
        println!("Interface #{}:", i);
        println!("  Path: {:?}", device_info.path());
        println!("  Manufacturer: {:?}", device_info.manufacturer_string());
        println!("  Product: {:?}", device_info.product_string());
        println!("  Serial: {:?}", device_info.serial_number());
        println!("  Release: {:04X}", device_info.release_number());
        println!("  Interface: {}", device_info.interface_number());
        println!("  Usage Page: 0x{:04X}", device_info.usage_page());
        println!("  Usage: 0x{:04X}", device_info.usage());
        println!();
    }
    
    Ok(())
}
