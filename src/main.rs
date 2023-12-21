use bpaf::Bpaf;
use std::{
    io::{self, Write},
    net::{IpAddr, Ipv4Addr},
    sync::mpsc::{channel, Sender},
    vec,
};
use tokio::net::TcpStream;

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// The address that you want to sniff. Must be a valid ipv4 address. Fallsback to 127.0.0.1
    pub addr: IpAddr,

    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Must be greater than 0")
    )]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,

    #[bpaf(long("end"), short('e'), fallback(MAX))]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    if TcpStream::connect((addr, port)).await.is_ok() {
        println!("Discovered open port: {}/tcp on {}", port, addr);
        io::stdout().flush().unwrap();
        tx.send(port).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let opts: Args = args().run();

    let (tx, rx) = channel::<u16>();
    for i in opts.start_port..=opts.end_port {
        let tx = tx.clone();
        tokio::task::spawn(async move { scan(tx, i, opts.addr).await });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("{:?}", out);
    out.sort();
    out.iter().for_each(|p| println!("{} is open", p))
}
