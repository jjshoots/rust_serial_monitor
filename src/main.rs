use std::env;
use std::io::{BufRead, BufReader};
use std::time::Duration;

fn main() {
    // this program accepts two arguments, a port identifier and a baud rate
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "This function must have 2 arguments - a port identifier and a baud_rate, got {}.",
        args.len() - 1
    );

    // get the port identifier and baud rate
    let identifier: &String = &args[1];
    let baud_rate: u32 = args[2]
        .parse()
        .expect("Expected second argument to be a unsigned number: ");

    let port = serialport::new(identifier, baud_rate)
        .timeout(Duration::from_millis(6000))
        .open()
        .expect("Error: ");

    // define the port as a bufreader that uses the port declared earlier
    let mut port = BufReader::new(port);

    // a storage buffer
    let mut buffer = String::new();

    println!("----------------------------------------------------------------------");
    println!("Serial monitor starting for port {} at {} baud rate.", identifier, baud_rate);
    println!("----------------------------------------------------------------------");

    loop {
        // clear line buffer, then read from port into it again
        buffer.clear();
        port.read_line(&mut buffer).expect("Read failed!");

        // print
        print!("{}", buffer);
    }
}
