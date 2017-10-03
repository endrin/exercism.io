#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(xs: &[T], ys: &[T]) -> Comparison
where
    T: PartialEq,
{
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
