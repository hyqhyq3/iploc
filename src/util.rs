use std::mem::transmute;
pub trait ToUl {
    fn to_ul(&self) -> Option<u32>;
}

impl ToUl for str {
    fn to_ul(&self) -> Option<u32> {
        let slices: Vec<_> = self.split(".").collect();
        match slices.len() {
            4 => {
                let mut res = 0u32;
                for s in slices {
                    let num = match s.parse::<u32>() {
                        Ok(n) => n,
                        Err(_) => {
                            return None;
                        }
                    };
                    res = res * 256 + num;
                }
                Some(res)
            }
            _ => None,
        }
    }
}

pub trait ToIp {
    fn to_ip(&self) -> String;
}

impl ToIp for u32 {
    fn to_ip(&self) -> String {
        let bytes: [u8; 4] = unsafe { transmute(self.to_be()) };
        let mut s = bytes.into_iter().map(|x| x.to_string() + ".").collect::<Vec<_>>().concat();
        s.pop();
        s
    }
}
