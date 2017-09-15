pub fn map_function(input: Vec<i32>, fxn: &Fn(i32) -> i32) -> Vec<i32> {
	let mut output: Vec<i32> = vec![0; input.len()];
	for (i, j) in input.iter().zip(output.iter_mut()) {
		*j = fxn(*i);
	}
	output
}

pub fn map_closure<F>(input: Vec<i32>, cls: F) -> Vec<i32> 
	where F: Fn(i32) -> i32 {
	let mut output: Vec<i32> = vec![0; input.len()];
	for (i, j) in input.iter().zip(output.iter_mut()) {
		*j = cls(*i);
	}
	output
}