#![feature(ip_from)]
#![feature(ip_as_octets)]

use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use libnetrangemerge::{merge_ranges, IpRange, Range, RangeInterest};
use tokio::io::AsyncWriteExt;
use crate::json_format::TorDetails;

mod json_format;

const ONION_API: &str = "https://onionoo.torproject.org/details?search=running:true";
const AVERAGE_IPV4_LIST_SIZE: usize = 7168;
const AVERAGE_IPV6_LIST_SIZE: usize = 3072;

async fn get_onion_details() -> anyhow::Result<(Vec<Ipv4Addr>, Vec<Ipv6Addr>)> {
    let client = reqwest::Client::builder().https_only(true).build()?;
    let api_result = client.get(ONION_API).send().await?;
    if !api_result.status().is_success() {
        return Err(anyhow::anyhow!("API request failed with code: {}", api_result.status()));
    }
    let body = api_result.bytes().await?;
    let details = serde_json::from_slice::<TorDetails>(&body)?;
    let mut ipv4_addresses = HashSet::with_capacity(AVERAGE_IPV4_LIST_SIZE);
    let mut ipv6_addresses = HashSet::with_capacity(AVERAGE_IPV6_LIST_SIZE);
    for relay in details.relays {
        for or_address in relay.or_addresses {
            let addr = if or_address.contains(':') {
                if or_address.starts_with('[') {
                    or_address.rsplit_once(':').unwrap().0.strip_prefix('[').unwrap().strip_suffix(']').unwrap()
                } else {
                    or_address.rsplit_once(':').unwrap().0
                }
            } else {
                &or_address
            };
            let parse_attempt = IpAddr::from_str(addr);
            if let Ok(ip_addr) = parse_attempt {
                if ip_addr.is_ipv4() {
                    ipv4_addresses.insert(Ipv4Addr::from_octets(ip_addr.as_octets().try_into()?));
                } else {
                    ipv6_addresses.insert(Ipv6Addr::from_octets(ip_addr.as_octets().try_into()?));
                }
            } else {
                eprintln!("Failed to parse IP address: {addr} == {or_address}");
            }
        }
    }

    let mut ipv4_addresses = ipv4_addresses.into_iter().collect::<Vec<_>>();
    let mut ipv6_addresses = ipv6_addresses.into_iter().collect::<Vec<_>>();
    ipv4_addresses.sort_unstable();
    ipv6_addresses.sort_unstable();
    
    Ok((ipv4_addresses, ipv6_addresses))
}


async fn generate_cidr_blocks(ipv4_addresses: Vec<Ipv4Addr>) -> anyhow::Result<Vec<IpRange>> {
    let mut ranges: Vec<RangeInterest<IpRange>> = Vec::with_capacity(ipv4_addresses.len());
    for address in ipv4_addresses {
        ranges.push(RangeInterest::new(format!("{address}/32").parse()?, true))
    }
    merge_ranges(&mut ranges);
    ranges.shrink_to_fit();
    ranges.sort_unstable_by(|a, b| a.range().host_address().cmp(b.range().host_address()));
    Ok(ranges.into_iter().map(|r| *r.range()).collect::<Vec<_>>())
}

async fn generate_cidr_blocks_v6(ipv6_addresses: Vec<Ipv6Addr>) -> anyhow::Result<Vec<IpRange>> {
    let mut ranges: Vec<RangeInterest<IpRange>> = Vec::with_capacity(ipv6_addresses.len());
    for address in ipv6_addresses {
        ranges.push(RangeInterest::new(format!("{address}/128").parse()?, true))
    }
    merge_ranges(&mut ranges);
    ranges.sort_unstable_by(|a, b| a.range().host_address().cmp(b.range().host_address()));
    Ok(ranges.into_iter().map(|r| *r.range()).collect::<Vec<_>>())
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let (ipv4_addresses, ipv6_addresses) = get_onion_details().await?;
    println!("found {} IPv4 addresses", ipv4_addresses.len());
    println!("found {} IPv6 addresses", ipv6_addresses.len());

    let ipv4_blocks = generate_cidr_blocks(ipv4_addresses).await?;
    println!("found {} IPv4 address blocks", ipv4_blocks.len());
    let ipv6_blocks = generate_cidr_blocks_v6(ipv6_addresses).await?;
    println!("found {} IPv6 address blocks", ipv6_blocks.len());

    let mut options = tokio::fs::OpenOptions::new();
    options.create(true).write(true).truncate(true);
    
    {
        let mut ipv4_file = options.clone().open("ipv4.txt").await?;
        for address_block in ipv4_blocks {
            if address_block.prefix_length() == 32 {
                ipv4_file.write_all(address_block.host_address().to_string().as_bytes()).await?;
                ipv4_file.write_all(b"\n").await?;
            } else {
                ipv4_file.write_all(address_block.to_string().as_bytes()).await?;
                ipv4_file.write_all(b"\n").await?;
            }
        }
        ipv4_file.flush().await?;
    }
    {
        let mut ipv6_file = options.open("ipv6.txt").await?;
        for address_block in ipv6_blocks {
            if address_block.prefix_length() == 128 {
                ipv6_file.write_all(address_block.host_address().to_string().as_bytes()).await?;
                ipv6_file.write_all(b"\n").await?;
            } else {
                ipv6_file.write_all(address_block.to_string().as_bytes()).await?;
                ipv6_file.write_all(b"\n").await?;
            }
        }
        ipv6_file.flush().await?;
    }
    
    
    Ok(())
}