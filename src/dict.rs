use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub enum Policy {
    Left, Right
}

pub struct Dict {
    wlst: Vec<Vec<char>>
}

impl Dict {
    pub fn load(path: &str) -> Result<Dict, &str> {
        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err("Cannot open dict")
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(_) => return Err("Cannot read dict")
        };
        let words = s.split("\n");
        let wlst: Vec<Vec<char>> = words.map(|w| w.chars().collect()).collect();
        Ok(Dict{wlst:wlst})
    }

    pub fn seek(&self, policy: Policy, _l: usize, _r: usize, offset: usize, ch: char) -> Option<usize> {
        let mut ans: usize = 0;
        let mut found = false;
        let mut m: usize;
        let mut l = _l as i64;
        let mut r = _r as i64;

        while l <= r {
            m = ((l + r) / 2) as usize;
            let w = &self.wlst[m];
            let wlen = w.len();
            if wlen <= offset {
                l = (m as i64) + 1;
            } else {
                let ch_ = w[offset];
                if ch_ < ch {
                    l = (m as i64) + 1;
                } else if ch_ > ch {
                    r = (m as i64) - 1;
                } else {
                    ans = m;
                    found = true;
                    match policy {
                        Policy::Left => r = (m as i64) - 1,
                        Policy::Right => l = (m as i64) + 1
                    }                    
                }
            }
        }
        if found {
            Some(ans)
        } else {
            None
        }
    }

    pub fn is_final(&self, len: usize, m: usize) -> bool {
        let w = &self.wlst[m];
        let wlen = w.len();
        wlen == len
    }

    pub fn r(&self) -> usize {
        self.wlst.len() - 1
    }
}
