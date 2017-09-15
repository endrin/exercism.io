macro_rules! generate_map {
    ($name: ident) => {
        pub fn $name<T, O>(collection: Vec<T>, operation: O) -> Vec<T>
                where O: Fn(T) -> T {
            let mut result = Vec::new();
            for v in collection {
                result.push(operation(v));
            }
            result
        }
    }
}

generate_map!(map_function);
generate_map!(map_closure);