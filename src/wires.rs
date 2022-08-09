// ---------------------------------------
// IOracle hardware
// ---------------------------------------
pub const SERIAL_DEV: &str = "/dev/ttyACM0";
pub const SERIAL_RATE: u32 = 9600;

pub fn read(
    _a: std::sync::Arc<std::sync::Mutex<Box<(dyn serialport::SerialPort)>>>,
    _multy: f32,
    _bias: f32,
    _threshold: f32,
) -> (u8, u8, u8) {
    let mut serial_buf: Vec<u8> = vec![0; 16];
    // ---------------------------------------
    // if let Ok(mut port) = port.lock() {
    //     println!("Serial: ok");
    //     if let Err(e) = port.read(serial_buf.as_mut_slice()) {
    //         println!("Found no data! {:?}", e);
    //     } else {
    //         println!("Read: {:?}", serial_buf);
    //     }
    // };
    // ---------------------------------------
    if let Ok(mut port) = serialport::new(SERIAL_DEV, SERIAL_RATE)
        .timeout(std::time::Duration::from_millis(100))
        .open()
    {
        println!("serial: ok");
        if let Err(e) = port.read(serial_buf.as_mut_slice()) {
            println!("Found no data! {:?}", e);
        } else {
            println!("ok2 {:?}", serial_buf);
        }
    };
    // ---------------------------------------

    (1, 1, 1)
}

pub fn react(trigram: (u8, u8, u8)) {
    println!("reaction! {:?}", trigram);
}
