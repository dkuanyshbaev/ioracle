// ---------------------------------------
// IOracle hardware
// ---------------------------------------
pub const SERIAL_DEV: &str = "/dev/ttyACM0";
pub const SERIAL_RATE: u32 = 9600;

pub fn read(
    port: std::sync::Arc<std::sync::Mutex<Box<(dyn serialport::SerialPort)>>>,
    multy: f32,
    bias: f32,
    threshold: f32,
) -> (u8, u8, u8) {
    // if let Ok(mut port) = serialport::new(SERIAL_DEV, SERIAL_RATE)
    //     .timeout(std::time::Duration::from_millis(100))
    //     .open()
    // {
    //     println!("serial: ok");
    //     let mut serial_buf: Vec<u8> = vec![0; 32];
    //     if let Err(e) = port.read(serial_buf.as_mut_slice()) {
    //         println!("Found no data! {:?}", e);
    //     } else {
    //         println!("ok2 {:?}", serial_buf);
    //     }
    // };
    //
    //
    // let mut serial_buf: Vec<u8> = vec![0; 1000];
    // println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
    // loop {
    //     match port.read(serial_buf.as_mut_slice()) {
    //         Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
    //         Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //         Err(e) => eprintln!("{:?}", e),
    //     }
    // }

    // let port = serialport::new(SERIAL_DEV, SERIAL_RATE)
    //     .open_native()
    //     .expect("Failed to open port");

    println!("----- {}, {}, {}", multy, bias, threshold);

    (1, 1, 1)
}

pub fn react(trigram: (u8, u8, u8)) {
    println!("reaction! {:?}", trigram);
}
