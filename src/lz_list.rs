use crate::get::Getable;
use crate::lz_err::LzErr;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Lz {
    pub name: String,
    pub deets: BTreeMap<String, String>,
}

impl Lz {
    pub fn new(s: &str) -> Self {
        let mut sp = s.split(|x| (x == ':') || (x == ','));
        let name = sp.next().expect("Split always has 1 result").to_string();
        let mut deets = BTreeMap::new();
        for (k, p) in sp.enumerate() {
            deets.insert(format!("ext{}", k), p.to_string());
        }
        Lz { name, deets }
    }

    pub fn add_deet_str(&mut self, s: &str) -> Result<(), LzErr> {
        let i = s.find(':').ok_or(LzErr::NotFound)?;
        let (k, v) = s.split_at(i);
        let v = &v[1..];
        self.add_deet(k, v);
        Ok(())
    }

    pub fn add_deet(&mut self, k: &str, v: &str) {
        self.deets
            .insert(k.trim().to_string(), v.trim().to_string());
    }
}

#[derive(Debug, Clone)]
pub struct LzList {
    pub items: Vec<Lz>,
}

impl FromStr for LzList {
    type Err = LzErr;
    fn from_str(s: &str) -> Result<Self, LzErr> {
        let sp = s.split(|x| (x == '\n') || (x == '\r'));
        let mut curr: Option<Lz> = None;
        let mut items = Vec::new();
        for (i, l) in sp.enumerate() {
            let tabbed = match l.chars().next() {
                Some('#') | None => continue,
                Some(c) => c.is_whitespace(),
            };
            if tabbed {
                let l = l.trim_start();
                match l.chars().next() {
                    Some('#') | None => continue,
                    _ => {}
                }

                match curr {
                    Some(ref mut curr) => curr.add_deet_str(l).map_err(|_| LzErr::ParseErrAt(i))?,
                    None => return Err(LzErr::ParseErrAt(i)),
                }

                continue;
            }
            if let Some(curr) = curr {
                items.push(curr);
            }
            curr = Some(Lz::new(l));
        }
        if let Some(c) = curr {
            items.push(c);
        }
        Ok(LzList { items })
    }
}

impl Getable for Lz {
    fn get(&self, s: &str) -> Option<String> {
        self.deets.get(s).map(|s| s.to_string())
    }
}

impl Getable for LzList {
    fn get(&self, s: &str) -> Option<String> {
        let sp: Vec<&str> = s.split(".").collect();
        match sp.len() {
            0 => return None,
            1 => self.items.get(0)?.get(sp[0]),
            _ => {
                for i in &self.items {
                    if i.name == sp[0] {
                        return i.get(sp[1]);
                    }
                }
                //CONSIDER passing remaining dots maybe even a dotgetter trait
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn make_lz() {
        let mut dave = Lz::new("dave,3,4");
        assert_eq!(dave.name, "dave");
        assert_eq!(dave.deets.get("ext0").unwrap(), "3");
        assert_eq!(dave.deets.get("ext1").unwrap(), "4");

        dave.add_deet_str(" fir : plop ").unwrap();
        assert_eq!(dave.deets.get("fir").unwrap(), "plop");
        assert!(dave.add_deet_str("fire").is_err());
        dave.add_deet_str(" flur:").unwrap();
        assert_eq!(dave.deets.get("flur").unwrap(), "");
    }

    #[test]
    fn make_lzlist() {
        let s = "\
superman:2
    power:fly
batman:5
    #
#poop
    power:money
";
        let lz = LzList::from_str(s).unwrap();
        assert_eq!(lz.items.len(), 2);
    }
}
