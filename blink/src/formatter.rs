use std::path::PathBuf;
use substring::Substring;

use crate::config;

pub(crate) struct Formatter
{	
	pub(crate) config : config::Config,
}

impl Formatter
{
	pub(crate) fn format(&self,path:&PathBuf,content:String)
	{
		let mut incorrect_curly_brackets = 0;

		for line in content.lines()
		{
			let cline = line.trim_end();
			if cline.ends_with("{")
			{
				let rline = cline.substring(0,cline.len()-1);
				let tline = rline.trim_start();
				let is_incorrect = tline.len() > 0;
				if is_incorrect
				{
					if self.config.verbose
					{
						println!("Found incorrect curly - {}",line);
					}
					incorrect_curly_brackets += 1;
				}
			}
		}

		println!("Summary for {}: ",path.display());
		println!("  Number of misplaced curlies: {}", incorrect_curly_brackets);
	}
}
