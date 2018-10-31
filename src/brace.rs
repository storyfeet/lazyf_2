use lz_err::LzErr;

fn _replace<IT>(it:&mut IT,f:&Fn(&str)->String,depth:u8)->Result<String,LzErr>
    where IT:Iterator<Item = char>
{
    let mut res = String::new();
    while let Some(c) = it.next(){
        match c {
            '\\'=>res.push(it.next().ok_or(LzErr::NotFound)?),
            '{'=>{
                let s = _replace(it,f,depth+1)?;
                res.push_str(&f(&s));
            },
            '}'=>{
                if depth == 0 { return Err(LzErr::NotFound)}
                return Ok(res);
            },
            c=>res.push(c),
        }
    }
    if depth > 0 {
        return Err(LzErr::NotFound);
    }
    Ok(res)
}

pub fn replace(s:&str,f:&Fn(&str)->String)->Result<String,LzErr>{
    _replace(&mut s.chars(),f,0)
}

#[cfg(test)]
mod test{
    use super::*;
    fn mini_rep(s:&str)->String{
        s.to_lowercase()
    }
    #[test]
    pub fn rep_test(){
        let s2 = replace("HELLO{WORLD}",&mini_rep).unwrap();
        assert_eq!(&s2,"HELLOworld");
    }
    
}
