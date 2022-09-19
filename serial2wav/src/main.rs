use std::i8;
use hound;

fn main() {
    // connect
    let ports = serialport::available_ports().expect("No ports found");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/cu.usbmodem141301", 115200)
        .open().expect("Failed to opne port");

    let mut serial_buf: Vec<u8> = vec![0; 1000];

    // wav
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 8000,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sample.wav", spec).unwrap();
    
    // counter
    let mut count = 0;

    loop {
        if let Ok(n) = port.read(serial_buf.as_mut_slice()) {
            for value in &serial_buf[..n] {
                println!("{}", value);
                writer.write_sample((*value as i16 - 147) as i8).unwrap();
            }
            count += n;
            if count > 80000 {
                break
            }
        }
    }
}
