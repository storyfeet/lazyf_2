use lz_err::LzErr;
use std::env;
use get::Getable;


pub struct EnvGetter();

impl Getable for EnvGetter{
    pub fn GetEnv(&self,s:&str)->Option<String>{
        env::var().ok()
    }
}

type Job<E> = Fn(&str)->Result<String,E>;

fn _replace<IT,E>(it:&mut IT,f:&Job<E>,depth:u8)->Result<String,LzErr>
    where IT:Iterator<Item = char>,
          //J:Job<E>,
          LzErr:From<E>
{
    let mut res = String::new();
    while let Some(c) = it.next(){
        match c {
            '\\'=>res.push(it.next().ok_or(LzErr::NotFound)?),
            '{'=>{
                let s = _replace(it,f,depth+1)?;
                res.push_str(&f(&s)?);
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

pub fn replace<E>(s:&str,f:&Job<E>)->Result<String,LzErr>
    where LzErr:From<E>,
          //J:Job<E>,
{
    _replace(&mut s.chars(),f,0)
}

pub fn replace_simple<F:'static+ Fn(&str)->String>(s:&str,f:F)->Result<String,LzErr>{
    replace::<LzErr>(s,& move|s|Ok(f(s)))
}

pub fn replace_env(s:&str)->Result<String,LzErr>{
    replace(s,&|v|env::var(v))
}

#[cfg(test)]
mod test{
    use super::*;
    fn mini_rep(s:&str)->String{
        s.to_lowercase()
    }
    #[test]
    pub fn rep_test(){
        let s2 = replace_simple("HELLO{WORLD}",&mini_rep).unwrap();
        assert_eq!(&s2,"HELLOworld");
    }
}
