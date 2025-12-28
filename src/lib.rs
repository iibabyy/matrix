// TODO: in arithmetics functions, instead of cloning and modifying the object,
// TODO: we should create an empty vector, and pushing the results of the operations into it
// TODO:
// TODO: for example, for add(a: &Vector<K>, b: &Vector<K>) -> Vector<K>, replace:
// TODO:
// TODO: ```Rust
// TODO: 	let mut new = a.clone();
// TODO: 	for i in 0..new.len() {
// TODO: 		new[i] += b[i];
// TODO: 	}
// TODO: 	return new
// TODO: ```
// TODO:
// TODO: by:
// TODO:
// TODO: ```Rust
// TODO: 	let mut new = Vec::with_capacity(a.len());
// TODO: 	for i in 0..a.len() {
// TODO: 		new.push(a[i] + b[i]);
// TODO: 	}
// TODO: 	Vector::new(new)
// TODO: ```

mod macros;
mod matrix;
mod vector;

mod functions;
pub use functions::*;
