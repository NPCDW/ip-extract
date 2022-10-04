#[allow(dead_code)]
fn ipv4_split_to_u32(split: Vec<&str>) -> Option<u32> {
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
pub fn ipv4_to_u32(ipv4: &str) -> Option<u32> {
    let split = ipv4.split(".").collect::<Vec<&str>>();
    ipv4_split_to_u32(split)
}

#[allow(dead_code)]
pub fn ipv6_to_u128(ipv6: &str) -> Option<u128> {
    let split = ipv6.split([':', '.'].as_ref()).collect::<Vec<&str>>();
    if split.len() == 4 {
        return Some((ipv4_split_to_u32(split)? as u128) | 0xffff00000000);
    }
    if split.len() != 8 {
        return None;
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
    if number < 281474976710656 {
        return u32_to_ipv4((number & 0xffffffff).try_into().unwrap());
    }
    let mut arr: [String; 8] = Default::default();
    for i in (0..8).rev() {
        arr[i] = format!("{:x}", (number & 0xffff));
        number = number >> 16;
    }
    arr.join(":")
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
        assert_eq!("0.0.0.0", u128_to_ipv6(281470681743360));
        assert_eq!("223.255.255.255", u128_to_ipv6(281474439839743));
        assert_eq!("255.255.255.255", u128_to_ipv6(281474976710655));
    }
}