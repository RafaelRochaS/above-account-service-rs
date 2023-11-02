#![allow(dead_code)]

use std::net::{IpAddr, Ipv4Addr};

pub const DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const DEFAULT_PORT: u16 = 50051;
