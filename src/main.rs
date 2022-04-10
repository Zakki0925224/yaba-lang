use std::io::stdin;

use yaba_lang::calc::expr_eval;

fn main()
{
    loop
    {
        let mut s = String::new();
        stdin().read_line(&mut s).ok();
        match expr_eval(&s)
        {
            Ok(v) => println!("ok:{}", v),
            Err(e) => println!("err:{}", e),
        }
    }
}
