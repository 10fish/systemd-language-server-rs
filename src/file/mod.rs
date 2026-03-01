use lazy_static::lazy_static;
use std::collections::HashMap;

mod automount;
mod device;
mod mount;
mod path;
mod scope;
mod service;
mod slice;
mod socket;
mod swap;
mod target;
mod timer;
mod unit;

use super::file::automount::AutomountFile;

pub enum Sytax<'s> {
    SectionName,
    DirectiveKey(&'s str),
    DirectiveValue(&'s str, &'s str),
}

pub trait Systemd: Sync {
    fn extention(&self) -> &'static str;

    fn required(&self, sytax: Sytax) -> &[&str];

    fn permits(&self, sytax: Sytax) -> &[&str];
}

lazy_static! {
    pub static ref SYSTEMD_UNIT_TYPES: HashMap<String, Box<dyn Systemd>> = {
        let mut map = HashMap::new();
        map.insert(
            "automount".to_string(),
            Box::new(AutomountFile {}) as Box<dyn Systemd + 'static>,
        );
        map
    };
}
