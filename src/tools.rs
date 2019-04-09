extern crate lazy_conf;
use clap::{clap_app, crate_version};
use std::str::FromStr;
use lazy_conf::LzList;

fn main() {
    let clap = clap_app!(
        lz_tools =>
            (about:"Get info about lz files")
            (version:crate_version!())
            (author:"Matthew Stoodley")
    //        (@arg count:-c "Count entries")
            (@arg files:-f +takes_value ... "The files to search")
    )
    .get_matches();

    let mut ll = Vec::new();
    for f in clap.values_of("files").unwrap() {
        let s = std::fs::read_to_string(f).expect(&format!("No file {}",f));
        let lzl = LzList::from_str(&s).unwrap();
        ll.extend(lzl.items);
    }

    println!("Total Entries = {}",ll.len());

}
