
use ec4rs::property::IndentStyle;
use std::path::Path;

pub(crate) fn load(verbose:bool,dryrun:bool,path:&Path) -> Config
{
	let default_config = Config
	{
		verbose,
		dryrun,
		indent_style: IndentStyle::Tabs,
		indent_size: 2,
		curly_brace_on_next_line : true,
	};

	let res = ec4rs::properties_of(path);

	match res
	{
		Ok(cfg) =>
		{
			let indent_style : IndentStyle = cfg.get::<IndentStyle>().unwrap_or(default_config.indent_style);
			let indent_size = cfg.get_raw_for_key("indent_size").into_str().parse::<u8>().unwrap_or(default_config.indent_size);
			let curly_brace_on_next_line = cfg.get_raw_for_key("curly_brace_on_next_line").into_str().parse::<bool>().unwrap_or(default_config.curly_brace_on_next_line);
			return Config
			{
				verbose,
				dryrun,
				indent_style,
				indent_size,
				curly_brace_on_next_line,
			};
		}

		Err(error) =>
		{
			println!("Warning: Unable to load editorconfig - Resorting to defaults\nReason: {}",error);
		}
	}

	return default_config;
}

#[derive(Debug,Copy,Clone)]
pub(crate) struct Config
{
	pub(crate) verbose: bool,
	pub(crate) dryrun: bool,

	indent_style : IndentStyle,
	indent_size : u8,
	pub(crate) curly_brace_on_next_line : bool
}