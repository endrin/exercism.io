pub struct Luhn(bool);

impl Luhn{
    pub fn is_valid(&self) -> bool {self.0}
}

impl<'x> std::convert::From<&'x str> for Luhn {
    fn from(inp: &str) -> Self {
        if inp.trim().len() > 1 {
            inp.chars().rev()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10))
                .collect::<Option<Luhn>>()
                .unwrap_or(Luhn(false))
        }else{
            Luhn(false)
        }
    }
}

impl std::convert::From<String> for Luhn {
    fn from(inp: String) -> Self {
        if inp.trim().len() > 1 {
            inp.chars().rev()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10))
                .collect::<Option<Luhn>>()
                .unwrap_or(Luhn(false))
        }else{
            Luhn(false)
        }
    }
}


macro_rules! impl_FromNum{
    ($($num:ty),*) => {$(  
        impl std::convert::From<$num> for Luhn {        
            fn from(inp: $num) -> Self  {
                if inp > 9 {
                    std::iter::repeat(())
                        .scan(inp, |val,_|{
                            if *val != 0 {
                                let res = *val % 10;
                                *val /= 10;
                                Some(res as u32)
                            }else{
                                None
                            }
                        })
                        .collect::<Luhn>()
                }else{
                    Luhn(false)
                }
            }
        }
    )*}
}


impl_FromNum!(u8, u16, u32, u64, usize);


fn fn0(x: u32) -> u32 {x}
fn fn1(x: u32) -> u32 { if x > 4 { (x << 1) - 9 } else { x << 1 } }

impl std::iter::FromIterator<u32> for Luhn {
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = u32>,
    {
        let sum = iter.into_iter()
            .zip([fn0, fn1].iter().cycle())
            .fold(0, |sum, (n, f)| sum + f(n));
        Luhn(sum % 10 == 0)
    }
}