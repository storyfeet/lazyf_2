extern crate lazy_conf;
use clap::{clap_app, crate_version};
use lazy_conf::{Getable, LzList};
use std::str::FromStr;

fn main() {
    let clap = clap_app!(
        lz_tools =>
            (about:"Get info about lz files")
            (version:crate_version!())
            (author:"Matthew Stoodley")
            (@arg count:-c +takes_value "Use Count entries")
            (@arg files:-f +takes_value ... "The files to search")
    )
    .get_matches();

    let mut ll = Vec::new();
    for f in clap.values_of("files").unwrap() {
        let s = std::fs::read_to_string(f).expect(&format!("No file {}", f));
        let lzl = LzList::from_str(&s).unwrap();
        ll.extend(lzl.items);
    }

    if let Some(ct) = clap.value_of("count") {
        let mut tot = 0;
        for l in &ll {
            tot += l.get(ct).map(|v| v.parse().unwrap_or(1)).unwrap_or(1);
        }
        println!("Counted Total = {}", tot);
    }

    println!("Total Entries = {}", ll.len());
}
