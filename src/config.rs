
use ec4rs::property::IndentStyle;
use std::{path::Path, fmt};

pub(crate) fn load(verbose:bool,dryrun:bool,path:&Path) -> Config
{
	fn load_properties_at_file_path(file_path:&Path) -> Option<ec4rs::Properties>
	{
		let res = ec4rs::properties_of(file_path);
		match res
		{
			Ok(cfg) =>
			{
				return Some(cfg);
			}
			Err(error) =>
			{
				println!("Warning: Unable to load editorconfig - Resorting to defaults\nReason: {}",error);
				return None;
			}
		}
	}

	fn load_properties(path:&Path) -> Option<ec4rs::Properties>
	{
		if path.is_dir()
		{
			return load_properties_at_file_path(&path.join("test.dart"));
		}

		let parent = path.parent();
		
		match parent
		{
			None => 
			{
				return None;
			}
			Some(p) => 
			{
				return load_properties_at_file_path(&p.join("test.dart"));
			}
		}
	}

	let default_config = Config
	{
		verbose,
		dryrun,
		indentation:Indentation { style:IndentationStyle::Tabs,size:2 },
		curly_brace_on_next_line : true,
	};

	let res = load_properties(path);

	match res
	{
		Some(cfg) =>
		{			
			let indent_style : IndentStyle = cfg.get::<IndentStyle>().unwrap_or(IndentStyle::Tabs);
			let indent_size = cfg.get_raw_for_key("indent_size").into_str().parse::<usize>().unwrap_or(default_config.indentation.size);
			let curly_brace_on_next_line = cfg.get_raw_for_key("curly_brace_on_next_line").into_str().parse::<bool>().unwrap_or(default_config.curly_brace_on_next_line);
			
			return Config
			{
				verbose,
				dryrun,
				indentation:load_indentation(indent_style,indent_size),
				curly_brace_on_next_line,
			};
		}

		None =>
		{
			return default_config;
		}
	}
}

fn load_indentation(style:IndentStyle,size:usize) -> Indentation
{
	match style
	{
		IndentStyle::Tabs => { return Indentation {style:IndentationStyle::Tabs,size}; }
		IndentStyle::Spaces => { return Indentation {style:IndentationStyle::Spaces,size}; }
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
	pub(crate) size: usize,
}

#[derive(Debug,Copy,Clone)]
pub(crate) enum IndentationStyle
{
	Tabs,
	Spaces,
}

impl std::fmt::Display for IndentationStyle
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		write!(f, "{:?}", self)
	}
}

impl Config
{
	pub(crate) fn display(&self) -> String
	{
		let mut s = String::from("");
		
		let p1 = format!("Curlies on next line: {}\n",self.curly_brace_on_next_line);
		s.push_str(p1.as_str());

		let p2 = format!("Indentation: {} - {}",self.indentation.style,self.indentation.size);
		s.push_str(p2.as_str());

		return s;
	}
}