use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration;

const RTSP_PORT: u16 = 554;
const SCAN_TIMEOUT: Duration = Duration::from_millis(300);

fn is_rtsp_open(ip: &str) -> bool {
    let addr = format!("{}:{}", ip, RTSP_PORT);

    match addr.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(socket_addr) = addrs.next() {
                TcpStream::connect_timeout(&socket_addr, SCAN_TIMEOUT).is_ok()
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub fn scan_subnet() -> Vec<String> {
    let mut found = Vec::new();

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(255);

        for octet in 1..=255u8 {
            let ip = format!("192.168.1.{}", octet);

            handles.push(scope.spawn(move || {
                let open = is_rtsp_open(&ip);
                (octet, ip, open)
            }));
        }

        for handle in handles {
            if let Ok((_, ip, open)) = handle.join() {
                if open {
                    found.push(ip);
                }
            }
        }
    });

    found
}
