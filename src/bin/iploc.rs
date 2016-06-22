extern crate iploc;
use std::env;

fn main() {
    let mut table = iploc::table::IpTable::new();
    table.init();

    for e in env::args().skip(1) {
        println!("{}", table.get_country(&e));
    }
}
