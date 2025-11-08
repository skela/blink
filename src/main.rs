use std::{
	collections::HashSet,
	io::{self, Write},
	path::PathBuf,
};

use clap::Parser;
use ignores::load_ignores;

mod config;
mod formatter;
mod ignores;
// mod treesitter;

fn main()
{
	let args = Arguments::parse();
	let config = config::load(args.verbose, args.dry_run, &args.path.as_path());
	// let config = config::load(args.verbose, args.dry_run, args.use_treesitter_to_format, &args.path.as_path());

	if args.check_config
	{
		println!("Config:");
		println!("{}", config.display());
		return;
	}

	if args.standard_input
	{
		format_standard_input(config);
	}
	// else if args.tree_sitter_format
	// {
	// 	let formatter = formatter::Formatter { config };
	// 	formatter.tree_sitter_format();
	// }
	// else if args.tree_sitter_analyze
	// {
	// 	let formatter = formatter::Formatter { config };
	// 	formatter.tree_sitter_analyze();
	// }
	else
	{
		format_files(config, &args.path, args.output);
	}
}

fn format_standard_input(config: config::Config)
{
	let formatter = formatter::Formatter { config };
	let mut buffer = String::new();
	let stdin = io::stdin();
	let lines = stdin.lines();
	for line_res in lines
	{
		match line_res
		{
			Ok(line) =>
			{
				buffer.push_str(line.as_str());
				buffer.push_str("\n");
			}
			Err(error) =>
			{
				eprintln!("Failed to read line - {}", error);
			}
		}
	}

	let result = formatter.format(buffer);

	let mut stdout = io::stdout().lock();
	let res = stdout.write_all(result.content.as_bytes());

	match res
	{
		Ok(_) =>
		{}
		Err(error) =>
		{
			eprintln!("Failed write to std out - {}", error);
		}
	}
}

fn format_files(config: config::Config, path: &PathBuf, output: Option<PathBuf>)
{
	let ignores = load_ignores(path);
	format_file_or_files_in_folder(config, &ignores, path, output)
}

fn format_file_or_files_in_folder(config: config::Config, ignores: &HashSet<PathBuf>, path: &PathBuf, output: Option<PathBuf>)
{
	if path.is_dir()
	{
		let res = std::fs::read_dir(&path);
		match res
		{
			Ok(paths) =>
			{
				for pr in paths
				{
					match pr
					{
						Ok(entry) =>
						{
							let entry_path = entry.path();
							if entry_path.is_dir()
							{
								if let Some(ref o) = output
								{
									if let Some(entry_file_name) = entry_path.file_name()
									{
										format_file_or_files_in_folder(config, ignores, &entry_path, Some(o.join(entry_file_name)));
									}
									else
									{
										println!("Failed to load filename for path {}", entry_path.display());
									}
								}
								else
								{
									format_file_or_files_in_folder(config, ignores, &entry_path, output.to_owned());
								}
							}
							else
							{
								format_file(config, &entry_path, ignores, output.to_owned());
							}
						}
						Err(err) =>
						{
							println!("Failed to check item in folder - {}", err);
						}
					}
				}
			}
			Err(error) =>
			{
				println!("Failed to list contents of folder at path `{}`\nReason: {}", path.display(), error);
			}
		}
	}
	else
	{
		format_file(config, &path, ignores, output.to_owned());
	}
}

fn format_file(config: config::Config, path: &PathBuf, ignores: &HashSet<PathBuf>, output_folder: Option<PathBuf>)
{
	let output = output_folder.unwrap_or(path.to_path_buf());
	if output.is_dir()
	{
		format_file_in_folder(config, path, ignores, &output);
	}
	else
	{
		let parent = output.parent().unwrap().to_path_buf();

		format_file_in_folder(config, path, ignores, &parent);
	}
}

fn format_file_in_folder(config: config::Config, path: &PathBuf, ignores: &HashSet<PathBuf>, output_folder: &PathBuf)
{
	let canonical_path = std::fs::canonicalize(path).unwrap();
	if path.extension().unwrap_or(std::ffi::OsStr::new("")) != "dart"
	{
		if config.verbose
		{
			println!("Skipping non dart file - {}", path.display());
		}
		return;
	}

	if path.to_string_lossy().ends_with("g.dart")
	{
		if config.verbose
		{
			println!("Skipping generated dart file - {}", path.display());
		}
		return;
	}
	if ignores.contains(&canonical_path)
	{
		if config.verbose
		{
			println!("Skipping ignored file - {}", path.display());
		}
		return;
	}

	if config.verbose
	{
		println!("Checking `{}`...", path.display());
	}

	let res = std::fs::read_to_string(path);

	match res
	{
		Ok(content) =>
		{
			let formatter = formatter::Formatter { config };
			let result = formatter.format(content);

			let fixed_path = output_folder.join(path.file_name().unwrap());

			if config.dryrun
			{
				println!("{}", result.content);

				println!("Stats for {} (wrongs): ", path.display());
				println!("  curlies: {} quotes: {} elses: {} indents: {} breaks: {}", result.incorrect_curly_braces, result.incorrect_quotes, result.incorrect_else_placements, result.incorrect_indentations, result.incorrect_break_placements);
			}
			else
			{
				let wres = std::fs::write(fixed_path, result.content);

				match wres
				{
					Ok(_) =>
					{}

					Err(err) =>
					{
						//println!("Error: Unable to write file `{}`\nReason: {}",fixed_path.display(),err);
						println!("Error: Unable to write file {}", err);
					}
				}
			}
		}

		Err(error) =>
		{
			println!("Error: Unable to read file `{}`\nReason: {}", path.display(), error);
		}
	}
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
/// A blazing fast code formatter for Dart
struct Arguments
{
	#[clap(short, long)]
	/// Output more detailed extra information
	verbose: bool,

	#[clap(short = 'd', long = "dry-run")]
	/// Output to the terminal only, don't make any changes
	dry_run: bool,

	#[clap(short = 's', long = "standard-input")]
	/// Standard input instead of path
	standard_input: bool,

	// #[clap(short = 'n', long = "use-treesitter")]
	/// Use new tree sitter formatter
	// use_treesitter_to_format: bool,

	// #[clap(short = 'a', long = "tree-sitter-analyze")]
	/// Test tree sitter analysis
	// tree_sitter_analyze: bool,

	// #[clap(short = 't', long = "tree-sitter-format")]
	/// Test tree sitter formatting
	// tree_sitter_format: bool,

	#[clap(short = 'c', long = "check-config")]
	/// Check the config, don't make any changes
	check_config: bool,

	#[clap(parse(from_os_str))]
	/// Path to input file or folder or working dir if using standard input
	path: std::path::PathBuf,

	#[clap(short, long, parse(from_os_str))]
	/// Path to output destination, overwrites files if omitted
	output: Option<std::path::PathBuf>,
}
