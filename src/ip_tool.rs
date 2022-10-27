#[allow(dead_code)]
pub fn ipv4_to_u32(ipv4: &str) -> Option<u32> {
    let split = ipv4.split(".").collect::<Vec<&str>>();
    if split.len() != 4 {
        return None;
    }
    let mut number: u32 = match split.get(0)?.parse::<u32>() {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    for i in 1..4 {
        number = number << 8;
        number = number | match split.get(i)?.parse::<u32>() {
            Err(_) => None,
            Ok(x) => Some(x),
        }?;
    }
    Some(number)
}

#[allow(dead_code)]
pub fn ipv6_to_u128(ipv6: &str) -> Option<u128> {
    if ipv6.contains(".") {
        let ipv4 = match ipv6.rfind(":") {
            None => ipv6,
            Some(pos) => &ipv6[pos+1..],
        };
        return Some((ipv4_to_u32(ipv4)? as u128) | 0xffff_0000_0000);
    }
    let mut split = ipv6.split([':'].as_ref()).collect::<Vec<&str>>();
    loop {
        let blank_pos = match split.iter().position(|&x| x == "") {
            None => break,
            Some(pos) => pos,
        };
        let _ = std::mem::replace(&mut split[blank_pos], "0");
        while split.len() != 8 {
            split.insert(blank_pos, "0");
        }
    }
    let mut number: u128 = match u128::from_str_radix(split.get(0)?, 16) {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    for i in 1..8 {
        number = number << 16;
        number = number | match u128::from_str_radix(split.get(i)?, 16) {
            Err(_) => None,
            Ok(x) => Some(x),
        }?;
    }
    Some(number)
}

#[allow(dead_code)]
pub fn u32_to_ipv4(mut number: u32) -> String {
    let mut arr: [String; 4] = Default::default();
    for i in (0..4).rev() {
        arr[i] = (number & 0xff).to_string();
        number = number >> 8;
    }
    arr.join(".")
}

#[allow(dead_code)]
pub fn u128_to_ipv6(mut number: u128) -> String {
    // if number >= 0xffff_0000_0000 && number <= 0xffff_ffff_ffff {
    //     return u32_to_ipv4((number & 0xffff_ffff).try_into().unwrap());
    // }
    let mut arr: [String; 8] = Default::default();
    for i in (0..8).rev() {
        arr[i] = format!("{:x}", (number & 0xffff));
        number = number >> 16;
    }
    arr.join(":")
}

#[allow(dead_code)]
pub fn ipv4_to_ipv6(ipv4: &str) -> Option<String> {
    let number = ipv4_to_u32(ipv4)?;
    let mut number = number as u128 | 0xffff_0000_0000;
    let mut arr: [String; 8] = Default::default();
    for i in (0..8).rev() {
        arr[i] = format!("{:x}", (number & 0xffff));
        number = number >> 16;
    }
    Some(arr.join(":"))
}

#[allow(dead_code)]
pub fn ipv6_to_ipv4(ipv6: &str) -> Option<String> {
    let number = ipv6_to_u128(ipv6)?;
    Some(u32_to_ipv4((number & 0xffff_ffff) as u32))
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct CidrIpv4Info {
    cidr: String,
    ip_start: String,
    ip_end: String,
    mask: String,
    count: u32,
}

#[allow(dead_code)]
pub fn cidr_to_ipv4(cidr: &str) -> Option<CidrIpv4Info> {
    let (ip, prefix) = cidr.split_at(cidr.rfind("/")?);
    let ip = ipv4_to_u32(ip)?;
    let prefix: u8 = match prefix[1..].parse::<u8>() {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    let mask = if prefix == 0 {
        u32::MIN
    } else if prefix == 32 {
        u32::MAX
    } else {
        ((1 << prefix) - 1) << (32 - prefix)
    };
    // 数量的范围为 1..(u32::MAX + 1)，最大值用 u32 放不下，用 0 表示
    let count: u32 = if prefix == 0 {
        0
    } else if prefix == 32 {
        1
    } else {
        1 << (32 - prefix)
    };
    let ip_start = ip & mask;
    let ip_end = ip | count.wrapping_sub(1);
    Some(CidrIpv4Info {
        cidr: cidr.to_string(),
        ip_start: u32_to_ipv4(ip_start),
        ip_end: u32_to_ipv4(ip_end),
        mask: u32_to_ipv4(mask),
        count,
    })
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct CidrIpv6Info {
    cidr: String,
    ip_start: String,
    ip_end: String,
    mask: String,
    count: u128,
}

#[allow(dead_code)]
pub fn cidr_to_ipv6(cidr: &str) -> Option<CidrIpv6Info> {
    let (ip, prefix) = cidr.split_at(cidr.rfind("/")?);
    let ip = ipv6_to_u128(ip)?;
    let prefix: u8 = match prefix[1..].parse::<u8>() {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    let mask = if prefix == 0 {
        u128::MIN
    } else if prefix == 128 {
        u128::MAX
    } else {
        ((1 << prefix) - 1) << (128 - prefix)
    };
    // 数量的范围为 1..(u128::MAX + 1)，最大值用 u128 放不下，用 0 表示
    let count: u128 = if prefix == 0 {
        0
    } else if prefix == 128 {
        1
    } else {
        1 << (128 - prefix)
    };
    let ip_start = ip & mask;
    let ip_end = ip | count.wrapping_sub(1);
    Some(CidrIpv6Info {
        cidr: cidr.to_string(),
        ip_start: u128_to_ipv6(ip_start),
        ip_end: u128_to_ipv6(ip_end),
        mask: u128_to_ipv6(mask),
        count,
    })
}

#[allow(dead_code)]
pub fn ipv4_to_cidr(ip_start: &str, ip_end: &str) -> Option<Vec<String>> {
    let mut ip_start = ipv4_to_u32(ip_start)?;
    let ip_end = ipv4_to_u32(ip_end)?;
    if ip_start > ip_end {
        return None;
    }
    let mut list = vec![];
    loop {
        let ip_start_b = format!("{:032b}", ip_start);
        let last_one_index = ip_start_b.rfind("1");
        let mut prefix = match last_one_index {
            None => 0,
            Some(x) => x + 1,
        };
        loop {
            let count: u32 = if prefix == 0 { 0 } else if prefix == 32 { 1 } else {
                1 << (32 - prefix)
            };
            let ip_start_mask_max_ip = ip_start | count.wrapping_sub(1);
            if ip_start_mask_max_ip == ip_end {
                let cidr = u32_to_ipv4(ip_start) + "/" + &prefix.to_string();
                list.push(cidr);
                return Some(list);
            } else if ip_start_mask_max_ip > ip_end {
                prefix += 1;
            } else {
                let cidr = u32_to_ipv4(ip_start) + "/" + &prefix.to_string();
                list.push(cidr);
                ip_start = ip_start_mask_max_ip + 1;
                break;
            }
        }
    }
}

#[allow(dead_code)]
pub fn ipv6_to_cidr(ip_start: &str, ip_end: &str) -> Option<Vec<String>> {
    let mut ip_start = ipv6_to_u128(ip_start)?;
    let ip_end = ipv6_to_u128(ip_end)?;
    if ip_start > ip_end {
        return None;
    }
    let mut list = vec![];
    loop {
        let ip_start_b = format!("{:0128b}", ip_start);
        let last_one_index = ip_start_b.rfind("1");
        let mut prefix = match last_one_index {
            None => 0,
            Some(x) => x + 1,
        };
        loop {
            let count: u128 = if prefix == 0 { 0 } else if prefix == 128 { 1 } else {
                1 << (128 - prefix)
            };
            let ip_start_mask_max_ip = ip_start | count.wrapping_sub(1);
            if ip_start_mask_max_ip == ip_end {
                let cidr = u128_to_ipv6(ip_start) + "/" + &prefix.to_string();
                list.push(cidr);
                return Some(list);
            } else if ip_start_mask_max_ip > ip_end {
                prefix += 1;
            } else {
                let cidr = u128_to_ipv6(ip_start) + "/" + &prefix.to_string();
                list.push(cidr);
                ip_start = ip_start_mask_max_ip + 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod ip_tool_test {
    use crate::ip_tool::*;

    #[test]
    fn ipv4_to_u32_test() {
        let x = ipv4_to_u32("223.255.255.255");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(3758096383, x.unwrap());
        let x = ipv4_to_u32("1.0.0.0");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(16777216, x.unwrap());
        let x = ipv4_to_u32("0.0.0.0");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(0, x.unwrap());
        let x = ipv4_to_u32("255.255.255.255");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(4294967295, x.unwrap());
    }
    
    #[test]
    fn ipv6_to_u128_test() {
        let x = ipv6_to_u128("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff");
        if x == None {
            panic!("convert ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff to u128 fail")
        }
        assert_eq!(340282366920938463463374607431768211455, x.unwrap());
        let x = ipv6_to_u128("2001:200:0:0:0:0:0:0");
        if x == None {
            panic!("convert 2001:200:0:0:0:0:0:0 to u128 fail")
        }
        assert_eq!(42540528726795050063891204319802818560, x.unwrap());
        let x = ipv6_to_u128("2001:200::1");
        if x == None {
            panic!("convert 2001:200::1 to u128 fail")
        }
        assert_eq!(42540528726795050063891204319802818561, x.unwrap());
        let x = ipv6_to_u128("::1");
        if x == None {
            panic!("convert ::1 to u128 fail")
        }
        assert_eq!(1, x.unwrap());
        let x = ipv6_to_u128("255.255.255.255");
        if x == None {
            panic!("convert 255.255.255.255 to u128 fail")
        }
        assert_eq!(281474976710655, x.unwrap());
        let x = ipv6_to_u128("0.0.0.0");
        if x == None {
            panic!("convert 0.0.0.0 to u128 fail")
        }
        assert_eq!(281470681743360, x.unwrap());
        let x = ipv6_to_u128("::0.0.0.0");
        if x == None {
            panic!("convert ::0.0.0.0 to u128 fail")
        }
        assert_eq!(281470681743360, x.unwrap());
        let x = ipv6_to_u128("223.255.255.255");
        if x == None {
            panic!("convert 223.255.255.255 to u128 fail")
        }
        assert_eq!(281474439839743, x.unwrap());
    }
    
    #[test]
    fn u32_to_ipv4_test() {
        assert_eq!("255.255.255.255", u32_to_ipv4(4294967295));
        assert_eq!("223.255.255.255", u32_to_ipv4(3758096383));
        assert_eq!("1.0.0.0", u32_to_ipv4(16777216));
        assert_eq!("0.0.0.0", u32_to_ipv4(0));
    }
    
    #[test]
    fn u128_to_ipv6_test() {
        assert_eq!("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff", u128_to_ipv6(340282366920938463463374607431768211455));
        assert_eq!("2001:200:0:0:0:0:0:0", u128_to_ipv6(42540528726795050063891204319802818560));
        assert_eq!("0:0:0:0:0:0:0:1", u128_to_ipv6(1));
        assert_eq!("0:0:0:0:0:0:0:0", u128_to_ipv6(0));
        // assert_eq!("0.0.0.0", u128_to_ipv6(281470681743360));
        // assert_eq!("223.255.255.255", u128_to_ipv6(281474439839743));
        // assert_eq!("255.255.255.255", u128_to_ipv6(281474976710655));
    }
    
    #[test]
    fn ipv4_to_ipv6_test() {
        assert_eq!(Some("0:0:0:0:0:ffff:ffff:ffff".to_string()), ipv4_to_ipv6("255.255.255.255"));
        assert_eq!(Some("0:0:0:0:0:ffff:3d80:8044".to_string()), ipv4_to_ipv6("61.128.128.68"));
        assert_eq!(Some("0:0:0:0:0:ffff:0:0".to_string()), ipv4_to_ipv6("0.0.0.0"));
    }
    
    #[test]
    fn ipv6_to_ipv4_test() {
        assert_eq!(Some("255.255.255.255".to_string()), ipv6_to_ipv4("0:0:0:0:0:ffff:ffff:ffff"));
        assert_eq!(Some("61.128.128.68".to_string()), ipv6_to_ipv4("0:0:0:0:0:ffff:3d80:8044"));
        assert_eq!(Some("0.0.0.0".to_string()), ipv6_to_ipv4("0:0:0:0:0:ffff:0:0"));
    }
    
    #[test]
    fn cidr_to_ipv4_test() {
        assert_eq!(CidrIpv4Info {
            cidr: "103.165.84.5/22".to_string(),
            ip_start: "103.165.84.0".to_string(),
            ip_end: "103.165.87.255".to_string(),
            mask: "255.255.252.0".to_string(),
            count: 1024,
        }, cidr_to_ipv4("103.165.84.5/22").unwrap());
        assert_eq!(CidrIpv4Info {
            cidr: "0.0.0.0/14".to_string(),
            ip_start: "0.0.0.0".to_string(),
            ip_end: "0.3.255.255".to_string(),
            mask: "255.252.0.0".to_string(),
            count: 262144,
        }, cidr_to_ipv4("0.0.0.0/14").unwrap());
        assert_eq!(CidrIpv4Info {
            cidr: "0.0.0.0/0".to_string(),
            ip_start: "0.0.0.0".to_string(),
            ip_end: "255.255.255.255".to_string(),
            mask: "0.0.0.0".to_string(),
            count: 0, // 最大为 4294967296 ，u32 放不下，被挤到 0
        }, cidr_to_ipv4("0.0.0.0/0").unwrap());
        assert_eq!(CidrIpv4Info {
            cidr: "1.1.1.1/32".to_string(),
            ip_start: "1.1.1.1".to_string(),
            ip_end: "1.1.1.1".to_string(),
            mask: "255.255.255.255".to_string(),
            count: 1,
        }, cidr_to_ipv4("1.1.1.1/32").unwrap());
    }
    
    #[test]
    fn cidr_to_ipv6_test() {
        assert_eq!(CidrIpv6Info {
            cidr: "CDCD:910A:2222:5498:8475:1111:3900:2020/64".to_string(),
            ip_start: "cdcd:910a:2222:5498:0:0:0:0".to_string(),
            ip_end: "cdcd:910a:2222:5498:ffff:ffff:ffff:ffff".to_string(),
            mask: "ffff:ffff:ffff:ffff:0:0:0:0".to_string(),
            count: 2_u128.pow(64),
        }, cidr_to_ipv6("CDCD:910A:2222:5498:8475:1111:3900:2020/64").unwrap());
        assert_eq!(CidrIpv6Info {
            cidr: "::0/0".to_string(),
            ip_start: "0:0:0:0:0:0:0:0".to_string(),
            ip_end: "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
            mask: "0:0:0:0:0:0:0:0".to_string(),
            count: 0, // 最大为 4294967296 ，u32 放不下，被挤到 0
        }, cidr_to_ipv6("::0/0").unwrap());
        assert_eq!(CidrIpv6Info {
            cidr: "::1/128".to_string(),
            ip_start: "0:0:0:0:0:0:0:1".to_string(),
            ip_end: "0:0:0:0:0:0:0:1".to_string(),
            mask: "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
            count: 1,
        }, cidr_to_ipv6("::1/128").unwrap());
    }
    
    #[test]
    fn ipv4_to_cidr_test() {
        assert_eq!(vec![
            "103.165.84.0/22".to_string(),
            "103.165.88.0/21".to_string(),
            "103.165.96.0/23".to_string(),
            "103.165.98.0/24".to_string(),
        ], ipv4_to_cidr("103.165.84.0", "103.165.98.255").unwrap());
        assert_eq!(vec![
            "192.168.6.73/32".to_string(),
            "192.168.6.74/31".to_string(),
            "192.168.6.76/30".to_string(),
            "192.168.6.80/28".to_string(),
            "192.168.6.96/27".to_string(),
            "192.168.6.128/30".to_string(),
            "192.168.6.132/32".to_string(),
        ], ipv4_to_cidr("192.168.6.73", "192.168.6.132").unwrap());
        assert_eq!(vec![
            "192.168.6.73/32".to_string(),
        ], ipv4_to_cidr("192.168.6.73", "192.168.6.73").unwrap());
        assert_eq!(vec![
            "0.0.0.0/0".to_string(),
        ], ipv4_to_cidr("0.0.0.0", "255.255.255.255").unwrap());
    }
    
    #[test]
    fn ipv6_to_cidr_test() {
        assert_eq!(vec![
            "16a0:10:ab00:1e:0:0:0:0/64".to_string(),
        ], ipv6_to_cidr("16A0:0010:AB00:001E:0000:0000:0000:0000", "16A0:0010:AB00:001E:FFFF:FFFF:FFFF:FFFF").unwrap());
        assert_eq!(vec![
            "0:0:0:0:0:0:0:0/0".to_string(),
        ], ipv6_to_cidr("::", "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap());
    }
}