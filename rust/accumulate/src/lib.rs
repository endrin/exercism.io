// Hey, their signatures are almost identical!

pub fn map_function<F, T, U>(seq: Vec<T>, f: &F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    let mut out = Vec::new();
    for val in seq {
        out.push(f(val))
    }
    out
}

pub fn map_closure<F, T, U>(seq: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    map_function(seq, &f)
}
