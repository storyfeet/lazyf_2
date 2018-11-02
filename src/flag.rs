use get::Getable;
use std::env;

pub struct FlagGetter();

fn _it_flag<I:Iterator<Item=String>>(mut it:I,s:&str)->Option<String>{
    while let Some(r) = it.next(){ 
        if r == s {
            return it.next();
        }
    }
    None
}

impl Getable for FlagGetter{
    fn get(&self,s:&str)->Option<String>{
        _it_flag(env::args(),s)
    }
    
    fn is_present(&self,s:&str)->bool{
        env::args().find(|v|v==s).is_some()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn flag_get_test(){
        let v = vec!["-a","3","-b"];
        assert_eq!(&_it_flag(v.iter().map(|s|s.to_string()),"-a").unwrap(),"3");
    }
}

