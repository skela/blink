use std::{path::PathBuf, ops::Add};
use substring::Substring;

use crate::config;

pub(crate) struct Formatter
{	
	pub(crate) config : config::Config,
}

impl Formatter
{
	pub(crate) fn format(&self,path:&PathBuf,content:String) -> String
	{
		let mut incorrect_curly_brackets = 0;
		let mut incorrect_indentations = 0;

		let mut line_number = 0;

		let mut fixedContent = String::from("");

		for line in content.lines()
		{
			let (fline1, changed1) = self.fix_incorrect_curly_brackets(line.trim_end().to_string());
			if changed1 { incorrect_curly_brackets += 1; }
			
			let (fline2,changed2) = self.fix_incorrect_indentation(fline1);
			if changed2 { incorrect_indentations += 1; }
			
			fixedContent.push_str(&fline2);
			fixedContent.push_str("\n");

			line_number += 1;
		}

		println!("Summary for {} (wrongs): ",path.display());
		println!("  curlies: {} indents: {}", incorrect_curly_brackets, incorrect_indentations);

		return fixedContent;
	}

	fn fix_incorrect_curly_brackets(&self,line:String) -> (String,bool)
	{
		if line.ends_with("{")
		{
			let line_length = line.len();
			let rline = line.substring(0,line_length-1);
			let tline = rline.trim_start();
			let is_incorrect = tline.len() > 0;
			if is_incorrect
			{
				let delta = line_length - line.trim_start().len();
				let mut s = String::from(rline.trim_end());
				if self.config.verbose
				{
					println!("Found incorrect curly - {}",line);
				}

				s.push_str("\n");
				s.push_str(line.substring(0,delta));
				s.push_str("{");

				return (s,true);
			}
		}
		return (line,false);
	}

	fn fix_incorrect_indentation(&self,line:String) -> (String,bool)
	{
		return (line,false);
	}
}
