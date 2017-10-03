#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<N, T, U>(xs: T, ys: U) -> Comparison
where
    N: Sized + PartialEq,
    T: AsRef<[N]>,
    U: AsRef<[N]>,
{
    let xs = xs.as_ref();
    let ys = ys.as_ref();

    if xs == ys {
        return Comparison::Equal;
    }

    if xs.is_empty() {
        return Comparison::Sublist;
    }

    if xs.len() > ys.len()
        && sublist(ys, xs) == Comparison::Sublist
    {
        return Comparison::Superlist;
    }

    if ys.windows(xs.len()).any(|y_subs| xs == y_subs) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
