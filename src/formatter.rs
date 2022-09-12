use std::path::PathBuf;
use substring::Substring;

use crate::config::{self, IndentationStyle};

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
			
			let (fline2,changed2) = self.fix_incorrect_indentation(fline1,self.config.verbose);
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
				
				if self.config.verbose
				{
					println!("Found incorrect curly - {}",line);
				}

				let mut s = String::from(rline.trim_end());
				s.push_str("\n");

				let mut s2 = String::from(line.substring(0,delta));
				s2.push_str("{");

				let (l,_) = self.fix_incorrect_indentation(s2,false);
				s.push_str(&l);

				return (s,true);
			}
		}
		return (line,false);
	}

	fn fix_incorrect_indentation(&self,line:String,verbose:bool) -> (String,bool)
	{
		match self.config.indentation.style
		{
			IndentationStyle::Tabs => 
			{
				let tline = line.trim_start();
				
				if tline.len() != line.len()
				{					
					let dspace = "  ";

					let index = line.find(tline).unwrap();
	
					let mut start = String::from(line.substring(0,index));
	
					if start.contains(dspace)
					{
						while start.contains(dspace)
						{
							start = start.replace(dspace,"\t");
						}

						if verbose
						{
							println!("Found incorrect indentation - {}",line);
						}

						start.push_str(tline);
						return (start,true);
					}
				}
			}

			IndentationStyle::Spaces =>
			{

			}
		}

		return (line,false);
	}
}
