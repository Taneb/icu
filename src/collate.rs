use libc::c_uchar;

// TODO: error enum

enum UCollator {}

#[link(name = "icuuc")]
extern {
    fn icu_ucol_open(loc: *const c_uchar, ec: *mut usize) -> *mut UCollator;
    fn icu_ucol_close(coll: *mut UCollator);
}

pub struct Collator {
    collator: *mut UCollator
}
    
impl Collator {
    pub fn open(loc: &str) -> Result<Collator, usize> {
        let mut err = 0;
        let r = unsafe {icu_ucol_open(loc.as_bytes().as_ptr(), &mut err)};
        match err {
            0 => Ok(Collator{collator: r}),
            _ => Err(err)
        }
    }

}

impl Drop for Collator {
    fn drop(&mut self) {
        unsafe {icu_ucol_close(self.collator)};
    }
}
