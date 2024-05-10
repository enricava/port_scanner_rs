use rayon::prelude::*;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
    u16,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 4);

    let ip = Ipv4Addr::from_str(&args[1]).expect("ip");
    let port_start = u16::from_str(&args[2]).expect("starting port");
    let port_end = u16::from_str(&args[3]).expect("ending port");

    const TIMEOUT: Duration = Duration::new(0, 10_000_000); // .1s

    let ports: Vec<u16> = (port_start..=port_end)
        .into_par_iter()
        .filter(|&port| {
            let addr = SocketAddr::new(IpAddr::V4(ip), port);
            TcpStream::connect_timeout(&addr, TIMEOUT).is_ok()
        })
        .collect();

    println!("Available ports: {:?}", ports);
}
