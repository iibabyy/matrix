use clap::Parser;
use ::matrix::matrix::functions::projection::projection;

fn main() {
	let args = Args::parse();
	let matrix = projection(args.fov, args.ratio, args.near, args.far);

	// prints one column by line
	for col in matrix.as_cols() {
		let vec = col.scalars().iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
		println!("{vec}");
	}
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
		#[arg(long, default_value_t = 1.0471976)]
		fov: f32,

		#[arg(long, default_value_t = 1.7777778)]
		ratio: f32,

		#[arg(long, default_value_t = 0.1)]
		near: f32,

		#[arg(long, default_value_t = 1000.0)]
		far: f32,
}
