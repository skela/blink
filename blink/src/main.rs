use clap::Parser;

fn main()
{
	let args = Arguments::parse();
	let res = std::fs::read_to_string(&args.path);

	match res
	{
		Ok(content) => 
		{
			for line in content.lines()
			{
				println!("{}", line);
			}
		}

		Err(error) => 
		{
			println!("Error: Unable to read file `{}`\nReason: {}",args.path.display(),error);
		}
	}
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
/// A blazing fast code formatter for Dart
struct Arguments
{
	#[clap(short,long)]
	/// Output more detailed extra information
	verbose: bool,

	#[clap(parse(from_os_str))]
	/// Path to input file
	path: std::path::PathBuf,

	#[clap(short,long,parse(from_os_str))]
	/// Path to output folder
	output: std::path::PathBuf,
}
