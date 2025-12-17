use std::error::Error;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::{env, io};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::timeout;

// todo 端口扫描器
const MAX: u16 = 65535;

#[derive(Debug)]
struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Self {
        let flag = args[1].clone();
        let ip_addr = args[2]
            .parse::<IpAddr>()
            .unwrap_or_else(|_| IpAddr::from_str("192.168.1.1").unwrap());
        let threads = args[3].parse::<u16>().unwrap_or_else(|_| 2000);
        Arguments {
            flag,
            ip_addr,
            threads,
        }
    }
    fn bind(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 2 {
            return Err(From::from("Too few arguments"));
        } else if args.len() > 4 {
            return Err(From::from("Too many arguments"));
        }
        Ok(Self::new(&args))
    }
}

#[tokio::main]
async fn main() {
    let env: Vec<_> = env::args().collect();
    if env.get(1).unwrap() == "-h" {
        show_help();
        return;
    }
    let args = Arguments::bind(&env).unwrap();
    let semaphore = Arc::new(Semaphore::new(args.threads.into()));
    let mut set = JoinSet::new();
    for i in 1..=MAX {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        set.spawn(async move {
            let _permit = permit;
            scan(args.ip_addr, i).await
        });
    }
    let mut out = vec![];
    let mut cont = 0;
    while let Some(res) = set.join_next().await {
        cont += 1;
        if let Ok(Some(port)) = res {
            out.push(port);
        }
        if cont % 1000 == 0 {
            print!("\r{}/{}", cont, MAX);
            io::stdout().flush().expect("TODO: panic message");
        }
    }
    out.sort();
    println!(
        "Open ports:\n{}",
        out.into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

async fn scan(ip_addr: IpAddr, port: u16) -> Option<u16> {
    match timeout(
        std::time::Duration::from_millis(300),
        TcpStream::connect(&SocketAddr::new(ip_addr, port)),
    )
    .await
    {
        Ok(Ok(_)) => Some(port),
        _ => None,
    }
}

fn show_help() {
    println!("Usage: \n 参数1: flag -h帮助 \n 参数2: ip_addr\n 参数3: threads");
}
