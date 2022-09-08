use std::{path::PathBuf};
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
		let mut incorrect_curly_braces = 0;
		let mut incorrect_indentations = 0;

		let forbidden_lines = self.forbidden_lines(&content);

		let mut line_number = 0;

		let mut fixed_content = String::from("");

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				fixed_content.push_str(&line);
				fixed_content.push_str("\n");
		
				line_number += 1;
				continue;
			}

			let (fline1, changed1) = self.fix_incorrect_curly_braces(line.trim_end().to_string());
			if changed1 { incorrect_curly_braces += 1; }
			
			let (fline2,changed2) = self.fix_incorrect_indentation(fline1);
			if changed2 { incorrect_indentations += 1; }
			
			fixed_content.push_str(&fline2);
			fixed_content.push_str("\n");

			line_number += 1;
		}

		println!("Summary for {} (wrongs): ",path.display());
		println!("  curlies: {} indents: {}", incorrect_curly_braces, incorrect_indentations);

		return fixed_content;
	}

	fn forbidden_lines(&self,content:&String) -> Vec<i32>
	{
		let mut forbidden : Vec<i32> = Vec::new();

		let mut line_number = 0;

		let mut is_inside_dquotes = false;
		let dquotes = "\"\"\"";

		for line in content.lines()
		{
			if line.contains(dquotes)
			{
				if is_inside_dquotes
				{
					is_inside_dquotes = false;
					forbidden.push(line_number);
				}
				else
				{
					is_inside_dquotes = true;
					forbidden.push(line_number);
				}
			}
			else if is_inside_dquotes
			{
				forbidden.push(line_number);
			}
			line_number += 1;
		}

		return forbidden;
	}

	fn fix_incorrect_curly_braces(&self,line:String) -> (String,bool)
	{
		if self.config.curly_brace_on_next_line && line.ends_with("{")
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
