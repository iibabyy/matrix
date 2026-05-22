use ::matrix::matrix::functions::projection::projection;

fn main() {
	#[cfg(feature = "clap")]
	let matrix = {
		use clap::Parser;
		let args = Args::parse();
		projection(args.fov, args.ratio, args.near, args.far)
	};

	#[cfg(not(feature = "clap"))]
	let matrix = projection(1.0471976, 1.7777778, 0.1, 1000.0);

	println!("{matrix}");
}

#[cfg(feature = "clap")]
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
