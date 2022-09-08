
use ec4rs::property::IndentStyle;
use std::path::Path;

pub(crate) fn load(verbose:bool,dryrun:bool,path:&Path) -> Config
{
	let default_config = Config
	{
		verbose,
		dryrun,
		indentation:Indentation { style:IndentationStyle::Tabs,size:2 },
		curly_brace_on_next_line : true,
	};

	let res = ec4rs::properties_of(path);

	match res
	{
		Ok(cfg) =>
		{
			let indent_style : IndentStyle = cfg.get::<IndentStyle>().unwrap_or(IndentStyle::Tabs);
			let indent_size = cfg.get_raw_for_key("indent_size").into_str().parse::<u8>().unwrap_or(2);
			let curly_brace_on_next_line = cfg.get_raw_for_key("curly_brace_on_next_line").into_str().parse::<bool>().unwrap_or(default_config.curly_brace_on_next_line);
			return Config
			{
				verbose,
				dryrun,
				indentation:load_indentation(indent_style,indent_size),
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

fn load_indentation(style:IndentStyle,size:u8) -> Indentation
{
	match style
	{
		IndentStyle::Tabs => { return Indentation {style:IndentationStyle::Tabs,size:size}; }
		IndentStyle::Spaces => { return Indentation {style:IndentationStyle::Spaces,size:size}; }
	}	
}

#[derive(Debug,Copy,Clone)]
pub(crate) struct Config
{
	pub(crate) verbose: bool,
	pub(crate) dryrun: bool,

	pub(crate) indentation : Indentation,	
	pub(crate) curly_brace_on_next_line : bool
}

#[derive(Debug,Copy,Clone)]
pub(crate) struct Indentation
{	
	pub(crate) style: IndentationStyle,
	pub(crate) size: u8,
}

#[derive(Debug,Copy,Clone)]
pub(crate) enum IndentationStyle
{
	Tabs,
	Spaces,
}
