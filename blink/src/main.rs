use std::path::PathBuf;

use clap::Parser;

mod config;
mod formatter;

fn main()
{
	let args = Arguments::parse();	
	let config = config::load(args.verbose,args.dryrun,&args.path.as_path());
	
	if args.path.is_dir()
	{
		let res = std::fs::read_dir(&args.path);
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
							format_file(config,&entry.path(),&args.output);
						}
						Err(err) =>
						{
							println!("Failed to check item in folder - {}",err);
						}
					}
				}
			}
			Err(error) =>
			{
				println!("Failed to list contents of folder at path `{}`\nReason: {}",args.path.display(),error);
			}
		}
	}
	else
	{
		format_file(config,&args.path,&args.output);
	}
}

fn format_file(config:config::Config,path:&PathBuf,output_folder:&PathBuf)
{
	if path.extension().unwrap_or(std::ffi::OsStr::new("")) != "dart"
	{
		if config.verbose
		{
			println!("Skipping non dart file - {}",path.display());
		}
		return
	}

	if config.verbose
	{
		println!("Checking `{}`...",path.display());
	}

	let res = std::fs::read_to_string(path);

	match res
	{
		Ok(content) => 
		{
			let formatter = formatter::Formatter { config, };
			let fixed_content = formatter.format(&path,content);

			let fixed_path = output_folder.join(path.file_name().unwrap());
						
			if config.dryrun
			{

			}
			else
			{
				let wres = std::fs::write(fixed_path, fixed_content);

				match wres
				{
					Ok(_) =>
					{

					}

					Err(err) =>
					{
						//println!("Error: Unable to write file `{}`\nReason: {}",fixed_path.display(),err);
						println!("Error: Unable to write file {}",err);
					}
				}
			}
		}

		Err(error) => 
		{
			println!("Error: Unable to read file `{}`\nReason: {}",path.display(),error);
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

	#[clap(short='d',long="dry-run")]
	/// Output to the terminal only, don't make any changes
	dryrun: bool,

	#[clap(parse(from_os_str))]
	/// Path to input file
	path: std::path::PathBuf,

	#[clap(short,long,parse(from_os_str))]
	/// Path to output folder
	output: std::path::PathBuf,
}
