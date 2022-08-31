use clap::Parser;
use ec4rs::property::IndentStyle;
use std::path::Path;

fn main()
{
	let args = Arguments::parse();
	let res = std::fs::read_to_string(&args.path);
	let config = load_config(&args.path.as_path());

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

fn load_config(path:&Path) -> Config
{
	let default_config = Config
	{
		indent_style: IndentStyle::Tabs,
		indent_size: 2,
		curly_bracket_next_line : true,
	};

	let res = ec4rs::properties_of(path);

	match res
	{
		Ok(cfg) =>
		{
			let indent_style : IndentStyle = cfg.get::<IndentStyle>().unwrap_or(default_config.indent_style);
			let indent_size = cfg.get_raw_for_key("indent_size").into_str().parse::<u8>().unwrap_or(default_config.indent_size);
			let curly_bracket_next_line = cfg.get_raw_for_key("curly_bracket_next_line").into_str().parse::<bool>().unwrap_or(default_config.curly_bracket_next_line);
			return Config
			{
				indent_style,
				indent_size,
				curly_bracket_next_line,
			};
		}

		Err(error) =>
		{
			println!("Warning: Unable to load editorconfig - Resorting to defaults\nReason: {}",error);
		}
	}

	return default_config;
}

struct Config
{
	indent_style : IndentStyle,
	indent_size : u8,
	curly_bracket_next_line : bool
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
