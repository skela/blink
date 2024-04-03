use substring::Substring;

use crate::config::{self, IndentationStyle};

pub(crate) struct Formatter
{
	pub(crate) config: config::Config,
}

pub(crate) struct FormatterResult
{
	pub(crate) content: String,
	pub(crate) incorrect_curly_braces: i32,
	pub(crate) incorrect_indentations: i32,
	pub(crate) incorrect_quotes: i32,
	pub(crate) incorrect_else_placements: i32,
	pub(crate) incorrect_break_placements: i32,
}

struct IncorrectSwitchBreakIndentation
{
	line: i32,
	indent: String,
}

impl Formatter
{
	pub(crate) fn format(&self, content: String) -> FormatterResult
	{
		let mut incorrect_curly_braces = 0;
		let mut incorrect_indentations = 0;
		let mut incorrect_quotes = 0;
		let mut incorrect_else_placements = 0;
		let mut incorrect_break_placements = 0;

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
			if changed1
			{
				incorrect_curly_braces += 1;
			}

			let (fline2, changed2) = self.fix_incorrect_indentation(fline1, self.config.verbose);
			if changed2
			{
				incorrect_indentations += 1;
			}

			let (fline3, changed3) = self.fix_incorrect_quotes(fline2);
			if changed3
			{
				incorrect_quotes += 1;
			}

			let (fline4, changed4) = self.fix_incorrect_else_placement(fline3);
			if changed4
			{
				incorrect_else_placements += 1;
			}

			let (fline5, changed5) = self.fix_incorrect_break_placement(fline4);
			if changed5
			{
				incorrect_break_placements += 1;
			}

			fixed_content.push_str(&fline5);
			fixed_content.push_str("\n");

			line_number += 1;
		}

		let cleaned_content1 = self.remove_repeating_empty_lines(&fixed_content);
		let cleaned_content2 = self.remove_preceeding_empty_lines(&cleaned_content1);
		let cleaned_content3 = self.correct_switch_break_indentations(&cleaned_content2);
		let cleaned_content4 = self.correct_weird_elses(&cleaned_content3);

		if self.config.use_treesitter_to_format
		{
			return FormatterResult { content: self.format_using_treesitter(cleaned_content4), incorrect_curly_braces, incorrect_indentations, incorrect_quotes, incorrect_else_placements, incorrect_break_placements };
		}

		return FormatterResult { content: cleaned_content4, incorrect_curly_braces, incorrect_indentations, incorrect_quotes, incorrect_else_placements, incorrect_break_placements };
	}

	fn forbidden_lines(&self, content: &String) -> Vec<i32>
	{
		let mut forbidden: Vec<i32> = Vec::new();

		let mut line_number = 0;

		let squotes = "'''";
		let mut is_inside_squotes = false;

		for line in content.lines()
		{
			if line.contains(squotes)
			{
				if is_inside_squotes
				{
					is_inside_squotes = false;
					forbidden.push(line_number);
				}
				else
				{
					is_inside_squotes = true;
					forbidden.push(line_number);
				}
			}
			else if is_inside_squotes
			{
				forbidden.push(line_number);
			}
			line_number += 1;
		}

		let mut is_inside_dquotes = false;
		let dquotes = "\"\"\"";
		line_number = 0;

		for line in content.lines()
		{
			if line.trim().starts_with("//")
			{
				forbidden.push(line_number);
				line_number += 1;
				continue;
			}
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

	fn empty_lines(&self, content: &String) -> Vec<i32>
	{
		let mut empty: Vec<i32> = Vec::new();

		let mut line_number = 0;

		for line in content.lines()
		{
			if line.is_empty() || line.trim().eq("{")
			{
				empty.push(line_number);
			}
			line_number += 1;
		}

		return empty;
	}

	fn empty_lines_preceeding_end_curly(&self, content: &String) -> Vec<i32>
	{
		let mut empty: Vec<i32> = Vec::new();

		let mut line_number = 0;

		let mut was_previous_line_empty = false;

		for line in content.lines()
		{
			if line.is_empty()
			{
				was_previous_line_empty = true;
			}
			else if was_previous_line_empty
			{
				if line.trim().eq("}")
				{
					empty.push(line_number - 1);
				}
				was_previous_line_empty = false;
			}

			line_number += 1;
		}

		return empty;
	}

	fn remove_repeating_empty_lines(&self, content: &String) -> String
	{
		let forbidden_lines = self.forbidden_lines(&content);
		let empty_lines = self.empty_lines(&content);

		let mut line_number = 0;
		let mut previous_line_number = -1;

		let mut cleaned_content = String::from("");

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				cleaned_content.push_str(&line);
				cleaned_content.push_str("\n");
				line_number += 1;
				previous_line_number += 1;
				continue;
			}

			if line_number > 0 && empty_lines.contains(&line_number) && empty_lines.contains(&previous_line_number)
			{
				line_number += 1;
				previous_line_number += 1;
				continue;
			}

			cleaned_content.push_str(&line);
			cleaned_content.push_str("\n");
			line_number += 1;
			previous_line_number += 1;
		}

		return cleaned_content;
	}

	fn remove_preceeding_empty_lines(&self, content: &String) -> String
	{
		let forbidden_lines = self.forbidden_lines(&content);
		let empty_preceed_lines = self.empty_lines_preceeding_end_curly(&content);

		let mut line_number = 0;

		let mut cleaned_content = String::from("");

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				cleaned_content.push_str(&line);
				cleaned_content.push_str("\n");
				line_number += 1;
				continue;
			}

			if empty_preceed_lines.contains(&line_number)
			{
				line_number += 1;
				continue;
			}

			cleaned_content.push_str(&line);
			cleaned_content.push_str("\n");
			line_number += 1;
		}

		return cleaned_content;
	}

	fn correct_weird_elses(&self, content: &String) -> String
	{
		let forbidden_lines = self.forbidden_lines(&content);

		let mut line_number = 0;

		let mut cleaned_content = String::from("");

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				cleaned_content.push_str(&line);
				cleaned_content.push_str("\n");
				line_number += 1;
				continue;
			}

			let (fixed_line, _) = self.fix_incorrect_else_placement(String::from(line));

			cleaned_content.push_str(fixed_line.as_str());
			cleaned_content.push_str("\n");
			line_number += 1;
		}

		return cleaned_content;
	}

	fn incorrect_switch_break_indendation_lines(&self, content: &String) -> Vec<IncorrectSwitchBreakIndentation>
	{
		let forbidden_lines = self.forbidden_lines(&content);

		let mut wrong: Vec<IncorrectSwitchBreakIndentation> = Vec::new();

		let mut line_number = 0;

		let mut last_case_line = -1;
		let mut last_case_line_indent = "";

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				line_number += 1;
				continue;
			}

			if last_case_line == -1 && line.trim().contains("case ") && !line.contains(" break;") && !line.contains(" return;") && !line.contains(" return ")
			{
				last_case_line = line_number;
				let delta = line.len() - line.trim_start().len();
				last_case_line_indent = line.substring(0, delta);
			}
			else
			{
				if last_case_line != -1
				{
					if line.trim().starts_with("break;")
					{
						wrong.push(IncorrectSwitchBreakIndentation { line: line_number, indent: String::from(last_case_line_indent) });
						last_case_line = -1;
					}
					else if line.trim().starts_with("return;") || line.contains(" return;") || line.contains(" return ")
					{
						last_case_line = -1;
					}
				}
			}

			line_number += 1;
		}

		return wrong;
	}

	fn correct_switch_break_indentations(&self, content: &String) -> String
	{
		let switch_breaks = self.incorrect_switch_break_indendation_lines(&content);

		let mut line_number = 0;

		let mut cleaned_content = String::from("");

		let mut correction_needed = false;

		for line in content.lines()
		{
			for br in &switch_breaks
			{
				if br.line == line_number
				{
					cleaned_content.push_str(&br.indent);
					cleaned_content.push_str(&line.trim_start());
					cleaned_content.push_str("\n");
					correction_needed = true;
					break;
				}
			}
			if !correction_needed
			{
				cleaned_content.push_str(&line);
				cleaned_content.push_str("\n");
			}
			correction_needed = false;
			line_number += 1;
		}

		return cleaned_content;
	}

	fn fix_incorrect_curly_braces(&self, line: String) -> (String, bool)
	{
		if self.config.curly_brace_on_next_line && line.ends_with("{")
		{
			let line_length = line.len();
			let rline = line.substring(0, line_length - 1);
			let tline = rline.trim_start();
			let is_incorrect = tline.len() > 0;
			if is_incorrect
			{
				let delta = line_length - line.trim_start().len();

				if self.config.verbose
				{
					println!("Found incorrect curly - {}", line);
				}

				let mut s = String::from(rline.trim_end());
				s.push_str("\n");

				let mut s2 = String::from(line.substring(0, delta));
				s2.push_str("{");

				let (l, _) = self.fix_incorrect_indentation(s2, false);
				s.push_str(&l);

				return (s, true);
			}
		}
		return (line, false);
	}

	fn fix_incorrect_else_placement(&self, line: String) -> (String, bool)
	{
		if self.config.curly_brace_on_next_line && line.contains("} else")
		{
			if line.ends_with("} else") || line.contains("} else ")
			{
				let line_length = line.len();
				let delta = line_length - line.trim_start().len();
				let pre = line.substring(0, delta);

				let mut s = String::from("}\n");
				s.push_str(pre);
				s.push_str("else");

				return (line.replace("} else", s.as_str()), true);
			}

			return (line, false);
		}
		return (line, false);
	}

	fn fix_incorrect_break_placement(&self, line: String) -> (String, bool)
	{
		if self.config.curly_brace_on_next_line && line.contains("} break;") && !line.contains("{")
		{
			if line.ends_with("} break;") || line.contains("} break; ")
			{
				let line_length = line.len();
				let delta = line_length - line.trim_start().len();
				let pre = line.substring(0, delta);

				let mut s = String::from("}\n");
				s.push_str(pre);
				s.push_str("break;");

				return (line.replace("} break;", s.as_str()), true);
			}

			return (line, false);
		}
		return (line, false);
	}

	fn fix_incorrect_indentation(&self, line: String, verbose: bool) -> (String, bool)
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

					let mut start = String::from(line.substring(0, index));

					if start.contains(dspace)
					{
						while start.contains(dspace)
						{
							start = start.replace(dspace, "\t");
						}

						if verbose
						{
							println!("Found incorrect indentation - {}", line);
						}

						start.push_str(tline);
						return (start, true);
					}
				}
			}

			IndentationStyle::Spaces =>
			{}
		}

		return (line, false);
	}

	fn fix_incorrect_quotes(&self, line: String) -> (String, bool)
	{
		if self.config.prefer_double_quotes && line.contains("'") && !line.starts_with("import '") && !line.starts_with("export '")
		{
			let mut number_of_singles = 0;
			let mut number_of_doubles = 0;
			for char in line.chars()
			{
				if char == '\''
				{
					number_of_singles += 1;
					continue;
				}
				if char == '"'
				{
					number_of_doubles += 1;
				}
			}

			if number_of_singles > 0 && number_of_singles % 2 == 0 && number_of_doubles == 0
			{
				return (line.replace("'", "\""), true);
			}
		}
		return (line, false);
	}
}
