use libc::*;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

// TODO: error enum

enum UCollator {}

#[link(name = "icui18n")]
extern "C" {
    fn ucol_open_52(loc: *const c_char, ec: *mut i32) -> *mut UCollator;
    fn ucol_close_52(coll: *mut UCollator);
    fn ucol_strcollUTF8_52(coll: *const UCollator,
                           source: *const c_char,
                           sourceLength: i32,
                           target: *const c_char,
                           sourceLength: i32,
                           status: *mut i32)
                           -> i32;
}

pub struct Collator {
    collator: *mut UCollator,
}

impl Collator {
    pub fn open(loc: &str) -> Result<Collator, i32> {
        let mut err = 0;
        let r = unsafe { ucol_open_52(loc.as_bytes().as_ptr() as *const c_char, &mut err) };
        if err <= 0 {
            Ok(Collator { collator: r })
        } else {
            Err(err)
        }
    }

    pub fn cmp(&self, source: &str, target: &str) -> Result<Ordering, i32> {
        let mut err = 0;
        let r = unsafe {
            ucol_strcollUTF8_52(self.collator,
                                source.as_bytes().as_ptr() as *const c_char,
                                source.as_bytes().len() as i32,
                                target.as_bytes().as_ptr() as *const c_char,
                                target.as_bytes().len() as i32,
                                &mut err)
        };
        if err <= 0 {
            Ok(match r {
                -1 => Less,
                1 => Greater,
                _ => Equal,
            })
        } else {
            Err(err)
        }
    }
}

impl Drop for Collator {
    fn drop(&mut self) {
        unsafe { ucol_close_52(self.collator) };
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::Collator;

    #[test]
    fn proof_of_concept() {
        let collator = Collator::open("en_US").unwrap();
        assert_eq!(collator.cmp("abc", "def").unwrap(), Ordering::Less);
    }

    #[test]
    fn czech_check() {
        let collator = Collator::open("cs_CZ").unwrap();
        assert_eq!(collator.cmp("chess", "hess").unwrap(), Ordering::Greater);

    }

}
