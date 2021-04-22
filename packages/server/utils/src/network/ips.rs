use get_if_addrs::get_if_addrs;
use std::io::Error;
use std::net::IpAddr;

pub fn get_all(sort: bool) -> Result<Vec<IpAddr>, Error> {
    let ifaces = get_if_addrs()?;
    let mut ips: Vec<IpAddr> = ifaces.iter().map(|iface| iface.addr.ip()).collect();
    if sort {
        ips.sort()
    }
    Ok(ips)
}
