use crate::config::{self, IndentationStyle};
use regex::Regex;
use std::collections::HashMap;
use substring::Substring;

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

struct SwitchLines
{
	start_line: i32,
	end_line: i32,
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
		let cleaned_content5 = self.unfold_trailing_comma_calls(&cleaned_content4);
		let cleaned_content6 = self.align_annotations(&cleaned_content5);

		// if self.config.use_treesitter_to_format
		// {
		// 	return FormatterResult { content: self.format_using_treesitter(cleaned_content6), incorrect_curly_braces, incorrect_indentations, incorrect_quotes, incorrect_else_placements, incorrect_break_placements };
		// }

		return FormatterResult { content: cleaned_content6, incorrect_curly_braces, incorrect_indentations, incorrect_quotes, incorrect_else_placements, incorrect_break_placements };
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

		let last_case_line = -1;

		let mut switches: Vec<SwitchLines> = Vec::new();
		let switch_regex = Regex::new(r"^\s*switch\s*\(.*\)\s*\{?\s*$").unwrap();
		let open_brace_regex = Regex::new(r"\{").unwrap();
		let close_brace_regex = Regex::new(r"\}").unwrap();

		let mut map: HashMap<i32, String> = HashMap::new();
		let mut stack: Vec<i32> = Vec::new();
		let mut braces: HashMap<i32, Vec<i32>> = HashMap::new();
		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				line_number += 1;
				continue;
			}

			if switch_regex.is_match(line)
			{
				let delta = line.len() - line.trim_start().len();
				let indent = line.substring(0, delta);
				stack.push(line_number);
				braces.insert(line_number, Vec::new());
				map.insert(line_number, indent.to_string());
			}

			if !stack.is_empty()
			{
				if open_brace_regex.is_match(line) && close_brace_regex.is_match(line)
				{
				}
				else
				{
					if open_brace_regex.is_match(line)
					{
						let stackb = braces.get_mut(stack.last().unwrap()).expect("Getting a braces stack for the switch block {");
						stackb.push(line_number);
					}

					if close_brace_regex.is_match(line)
					{
						let stackb = braces.get_mut(stack.last().unwrap()).expect("Getting a braces stack for the switch block }");
						stackb.pop();
						if stackb.is_empty()
						{
							if let Some(start_line) = stack.pop()
							{
								let indent = map.get(&start_line).unwrap();
								switches.push(SwitchLines { start_line, end_line: line_number, indent: indent.to_string() });
							}
						}
					}
				}
			}
			line_number += 1;
		}

		line_number = 0;
		let mut active_switch: Option<usize> = None;
		let mut active_switches: Vec<&SwitchLines> = Vec::new();
		switches.sort_by_key(|switch| switch.start_line);
		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				line_number += 1;
				continue;
			}
			if switch_regex.is_match(line)
			{
				if let Some(index) = active_switch
				{
					active_switch = Some(index + 1);
				}
				else
				{
					active_switch = Some(0);
				}
				if let Some(sw) = switches.get(active_switch.unwrap())
				{
					active_switches.push(sw);
				}
			}
			else
			{
				if let Some(sw) = active_switches.last()
				{
					if line.trim().contains("case ") && !line.contains(" break;") && !line.contains(" return;") && !line.contains(" return ")
					{
					}
					else
					{
						if line.trim().starts_with("break;")
						{
							wrong.push(IncorrectSwitchBreakIndentation { line: line_number, indent: sw.indent.to_string() + "\t" });
						}
						else if line.trim().starts_with("return;") || line.contains(" return;") || line.contains(" return ")
						{
						}
					}
					if sw.end_line == line_number
					{
						active_switches.pop();
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

	fn indent_unit(&self) -> String
	{
		match self.config.indentation.style
		{
			IndentationStyle::Tabs => String::from("\t"),
			IndentationStyle::Spaces => " ".repeat(self.config.indentation.size),
		}
	}

	fn unfold_trailing_comma_calls(&self, content: &String) -> String
	{
		let forbidden_lines = self.forbidden_lines(&content);

		let mut line_number: i32 = 0;
		let mut cleaned_content = String::new();

		let mut collecting_segment = false;
		let mut segment_buffer = String::new();
		let mut segment_balance = 0;

		for line in content.lines()
		{
			if forbidden_lines.contains(&line_number)
			{
				if collecting_segment && !segment_buffer.is_empty()
				{
					cleaned_content.push_str(segment_buffer.as_str());
					segment_buffer.clear();
					collecting_segment = false;
					segment_balance = 0;
				}

				cleaned_content.push_str(line);
				cleaned_content.push('\n');
				line_number += 1;
				continue;
			}

			if collecting_segment
			{
				segment_buffer.push_str(line);
				segment_buffer.push('\n');

				let (delta, _) = self.paren_balance_delta(line);
				segment_balance += delta;

				if segment_balance <= 0
				{
					if let Some(unfolded) = self.unfold_trailing_comma_segment(segment_buffer.as_str())
					{
						cleaned_content.push_str(unfolded.as_str());
					}
					else
					{
						cleaned_content.push_str(segment_buffer.as_str());
					}

					segment_buffer.clear();
					collecting_segment = false;
					segment_balance = 0;
				}
			}
			else
			{
				match self.unfold_trailing_comma_line(line)
				{
					Some(unfolded) =>
					{
						cleaned_content.push_str(unfolded.as_str());
					}
					None =>
					{
						let (delta, has_open) = self.paren_balance_delta(line);

						if delta > 0 && has_open
						{
							collecting_segment = true;
							segment_balance = delta;
							segment_buffer.push_str(line);
							segment_buffer.push('\n');
						}
						else
						{
							cleaned_content.push_str(line);
							cleaned_content.push('\n');
						}
					}
				}
			}

			line_number += 1;
		}

		if collecting_segment && !segment_buffer.is_empty()
		{
			if let Some(unfolded) = self.unfold_trailing_comma_segment(segment_buffer.as_str())
			{
				cleaned_content.push_str(unfolded.as_str());
			}
			else
			{
				cleaned_content.push_str(segment_buffer.as_str());
			}
		}

		return cleaned_content;
	}

	fn align_annotations(&self, content: &String) -> String
	{
		let forbidden_lines = self.forbidden_lines(content);

		let mut result = String::new();
		let mut line_number: i32 = 0;

		let lines: Vec<&str> = content.lines().collect();
		let total_lines = lines.len();

		while line_number < total_lines as i32
		{
			let line = lines[line_number as usize];

			if forbidden_lines.contains(&line_number)
			{
				result.push_str(line);
				result.push('\n');
				line_number += 1;
				continue;
			}

			if line.trim_start().starts_with('@')
			{
				if let Some(next_line) = lines.get((line_number + 1) as usize)
				{
					let indent: String = next_line.chars().take_while(|c| c.is_whitespace()).collect();
					let mut target_indent = indent;

					if target_indent.is_empty()
					{
						target_indent = line.chars().take_while(|c| c.is_whitespace()).collect();
					}

					result.push_str(target_indent.as_str());
					result.push_str(line.trim_start());
					result.push('\n');

					line_number += 1;
					continue;
				}
			}

			result.push_str(line);
			result.push('\n');
			line_number += 1;
		}

		return result;
	}

	fn unfold_trailing_comma_line(&self, line: &str) -> Option<String>
	{
		if !line.contains('(') || !line.contains(')') || line.trim().is_empty()
		{
			return None;
		}

		let first_paren = line.find('(')?;
		let chars: Vec<char> = line.chars().collect();

		let closing_paren = self.find_matching_paren(&chars, first_paren)?;

		let before_closing = &line[..closing_paren];

		let leading_indent_len = line.chars().take_while(|c| c.is_whitespace()).count();
		let leading_indent = &line[..leading_indent_len];
		let before_args = &line[..first_paren];
		let suffix = &line[closing_paren + 1..];
		let args_section = &line[first_paren + 1..closing_paren];

		let arguments = self.split_arguments(args_section);
		if arguments.is_empty()
		{
			return None;
		}

		if !self.contains_trailing_comma(before_closing, &arguments)
		{
			return None;
		}

		let mut unfolded = String::new();
		unfolded.push_str(before_args);
		unfolded.push_str("(\n");

		let mut param_indent = String::from(leading_indent);
		param_indent.push_str(self.indent_unit().as_str());

		for argument in arguments
		{
			let trimmed_argument = argument.trim();
			if trimmed_argument.is_empty()
			{
				continue;
			}

			let formatted_lines = self.format_argument_lines(trimmed_argument);
			let total_lines = formatted_lines.len();

			for (index, formatted_line) in formatted_lines.iter().enumerate()
			{
				unfolded.push_str(param_indent.as_str());
				unfolded.push_str(formatted_line);

				if index == total_lines - 1
				{
					unfolded.push_str(",\n");
				}
				else
				{
					unfolded.push('\n');
				}
			}
		}

		unfolded.push_str(leading_indent);
		unfolded.push(')');
		unfolded.push_str(suffix);
		unfolded.push('\n');

		return Some(unfolded);
	}

	fn unfold_trailing_comma_segment(&self, segment: &str) -> Option<String>
	{
		let trimmed_segment = segment.trim_end_matches('\n');
		let chars: Vec<char> = trimmed_segment.chars().collect();

		let (open_index, close_index) = self.find_outermost_paren_indices(&chars)?;

		let before_args = &trimmed_segment[..open_index];
		let suffix = &trimmed_segment[close_index + 1..];
		let args_section = &trimmed_segment[open_index + 1..close_index];

		let arguments = self.split_arguments(args_section);
		if arguments.is_empty()
		{
			return None;
		}

		if !self.contains_trailing_comma(&trimmed_segment[..close_index], &arguments)
		{
			return None;
		}

		let leading_indent_len = before_args.chars().take_while(|c| c.is_whitespace()).count();
		let leading_indent = &before_args[..leading_indent_len];
		let before_args_trimmed = before_args.trim_end();

		let mut unfolded = String::new();
		unfolded.push_str(before_args_trimmed);
		unfolded.push_str("(\n");

		let mut param_indent = String::from(leading_indent);
		param_indent.push_str(self.indent_unit().as_str());

		for argument in arguments
		{
			let trimmed_argument = argument.trim();
			if trimmed_argument.is_empty()
			{
				continue;
			}

			let formatted_lines = self.format_argument_lines(trimmed_argument);
			let total_lines = formatted_lines.len();

			for (index, formatted_line) in formatted_lines.iter().enumerate()
			{
				unfolded.push_str(param_indent.as_str());
				unfolded.push_str(formatted_line);

				let has_trailing_comma = formatted_line.trim_end().ends_with(',');

				if index == total_lines - 1 && !has_trailing_comma
				{
					unfolded.push(',');
				}

				unfolded.push('\n');
			}
		}

		unfolded.push_str(leading_indent);
		unfolded.push(')');
		unfolded.push_str(suffix.trim_end_matches('\n'));
		unfolded.push('\n');

		return Some(unfolded);
	}

	fn find_matching_paren(&self, chars: &[char], start_index: usize) -> Option<usize>
	{
		let mut depth = 0;
		let mut index = start_index;

		while index < chars.len()
		{
			match chars[index]
			{
				'(' =>
				{
					depth += 1;
				}
				')' =>
				{
					if depth == 0
					{
						return None;
					}

					depth -= 1;
					if depth == 0
					{
						return Some(index);
					}
				}
				_ => {}
			}
			index += 1;
		}

		return None;
	}

	fn split_arguments(&self, args_section: &str) -> Vec<String>
	{
		let mut trimmed_chars: Vec<char> = args_section.trim().chars().collect();

		while trimmed_chars.last().map(|c| c.is_whitespace()).unwrap_or(false)
		{
			trimmed_chars.pop();
		}

		if trimmed_chars.last() == Some(&',')
		{
			trimmed_chars.pop();
		}

		let core: String = trimmed_chars.into_iter().collect();

		let core_chars: Vec<char> = core.chars().collect();
		let mut arguments: Vec<String> = Vec::new();
		let mut current = String::new();

		let mut paren_depth = 0;
		let mut brace_depth = 0;
		let mut bracket_depth = 0;
		let mut angle_depth = 0;
		let mut in_single_quote = false;
		let mut in_double_quote = false;
		let mut prev_non_ws: Option<char> = None;

		let len = core_chars.len();
		let mut index = 0;

		while index < len
		{
			let ch = core_chars[index];

			if !in_single_quote && !in_double_quote
			{
				match ch
				{
					'(' => paren_depth += 1,
					')' =>
					{
						if paren_depth > 0
						{
							paren_depth -= 1;
						}
					}
					'{' => brace_depth += 1,
					'}' =>
					{
						if brace_depth > 0
						{
							brace_depth -= 1;
						}
					}
					'[' => bracket_depth += 1,
					']' =>
					{
						if bracket_depth > 0
						{
							bracket_depth -= 1;
						}
					}
					'<' =>
					{
						let next_non_ws = core_chars[index + 1..].iter().find(|c| !c.is_whitespace()).copied();
						if prev_non_ws.map(|c| c.is_alphanumeric() || c == '>' || c == '?').unwrap_or(false)
							&& next_non_ws.map(|c| c.is_alphanumeric() || c == '_' || c == '?' || c == '>').unwrap_or(false)
						{
							angle_depth += 1;
						}
					}
					'>' =>
					{
						if angle_depth > 0
						{
							angle_depth -= 1;
						}
					}
					_ => {}
				}
			}

			match ch
			{
				'\'' =>
				{
					if !in_double_quote && !self.is_escaped(&core_chars, index)
					{
						in_single_quote = !in_single_quote;
					}
					current.push(ch);
				}
				'"' =>
				{
					if !in_single_quote && !self.is_escaped(&core_chars, index)
					{
						in_double_quote = !in_double_quote;
					}
					current.push(ch);
				}
				',' =>
				{
					if paren_depth == 0 && brace_depth == 0 && bracket_depth == 0 && angle_depth == 0 && !in_single_quote && !in_double_quote
					{
						arguments.push(current.trim().to_string());
						current.clear();
					}
					else
					{
						current.push(ch);
					}
				}
				_ =>
				{
					current.push(ch);
				}
			}

			if !ch.is_whitespace()
			{
				prev_non_ws = Some(ch);
			}

			index += 1;
		}

		if !current.trim().is_empty()
		{
			arguments.push(current.trim().to_string());
		}

		return arguments;
	}

	fn format_argument_lines(&self, argument: &str) -> Vec<String>
	{
		if let Some(lines) = self.format_bracket_expression_lines(argument)
		{
			return lines;
		}

		if let Some(lines) = self.format_parenthesized_expression_lines(argument)
		{
			return lines;
		}

		return vec![argument.trim().to_string()];
	}

	fn format_bracket_expression_lines(&self, argument: &str) -> Option<Vec<String>>
	{
		let chars: Vec<char> = argument.chars().collect();
		let (open_index, close_index) = self.find_top_level_bracket_indices(&chars)?;

		let prefix = &argument[..open_index];
		let suffix = argument[close_index + 1..].trim();
		let items_section = &argument[open_index + 1..close_index];
		let items_trimmed = items_section.trim();

		if items_trimmed.is_empty() || !items_trimmed.ends_with(',')
		{
			return None;
		}

		let items = self.split_arguments(items_section);
		if items.is_empty()
		{
			return None;
		}

		let indent_unit = self.indent_unit();

		let mut lines: Vec<String> = Vec::new();

		let mut first_line = String::from(prefix);
		first_line.push('[');
		lines.push(first_line);

		for item in items
		{
			let trimmed_item = item.trim();
			if trimmed_item.is_empty()
			{
				continue;
			}

			let nested_lines = self.format_argument_lines(trimmed_item);
			let total_nested = nested_lines.len();

			for (index, nested_line) in nested_lines.iter().enumerate()
			{
				let mut line = String::new();
				line.push_str(indent_unit.as_str());
				line.push_str(nested_line);

				if index == total_nested - 1 && !nested_line.trim_end().ends_with(',')
				{
					line.push(',');
				}

				lines.push(line);
			}
		}

		let mut closing = String::from("]");
		if !suffix.is_empty()
		{
			closing.push_str(suffix);
		}
		lines.push(closing);

		return Some(lines);
	}

	fn format_parenthesized_expression_lines(&self, argument: &str) -> Option<Vec<String>>
	{
		let chars: Vec<char> = argument.chars().collect();
		let open_index = argument.find('(')?;
		let close_index = self.find_matching_paren(&chars, open_index)?;

		let before_closing = &argument[..close_index];
		let items_section = &argument[open_index + 1..close_index];

		let items = self.split_arguments(items_section);

		if items.is_empty()
		{
			return None;
		}

		if !self.contains_trailing_comma(before_closing, &items)
		{
			return None;
		}

		let prefix = &argument[..open_index];
		let suffix = argument[close_index + 1..].trim();

		let indent_unit = self.indent_unit();

		let mut lines: Vec<String> = Vec::new();

		let mut first_line = String::from(prefix.trim_end());
		first_line.push('(');
		lines.push(first_line);

		for item in items
		{
			let trimmed_item = item.trim();
			if trimmed_item.is_empty()
			{
				continue;
			}

			let nested_lines = self.format_argument_lines(trimmed_item);
			let total_nested = nested_lines.len();

			for (index, nested_line) in nested_lines.iter().enumerate()
			{
				let mut line = String::new();
				line.push_str(indent_unit.as_str());
				line.push_str(nested_line);

				if index == total_nested - 1 && !nested_line.trim_end().ends_with(',')
				{
					line.push(',');
				}

				lines.push(line);
			}
		}

		let mut closing = String::from(")");
		if !suffix.is_empty()
		{
			closing.push_str(suffix);
		}
		lines.push(closing);

		return Some(lines);
	}

	fn find_top_level_bracket_indices(&self, chars: &[char]) -> Option<(usize, usize)>
	{
		let mut paren_depth = 0;
		let mut brace_depth = 0;
		let mut bracket_depth = 0;
		let mut angle_depth = 0;
		let mut in_single_quote = false;
		let mut in_double_quote = false;
		let mut prev_non_ws: Option<char> = None;

		let mut open_index: Option<usize> = None;

		let len = chars.len();
		let mut index = 0;

		while index < len
		{
			let ch = chars[index];

			match ch
			{
				'\'' =>
				{
					if !in_double_quote && !self.is_escaped(chars, index)
					{
						in_single_quote = !in_single_quote;
					}
				}
				'"' =>
				{
					if !in_single_quote && !self.is_escaped(chars, index)
					{
						in_double_quote = !in_double_quote;
					}
				}
				_ => {}
			}

			if in_single_quote || in_double_quote
			{
				index += 1;
				continue;
			}

			match ch
			{
				'(' => paren_depth += 1,
				')' =>
				{
					if paren_depth > 0
					{
						paren_depth -= 1;
					}
				}
				'{' => brace_depth += 1,
				'}' =>
				{
					if brace_depth > 0
					{
						brace_depth -= 1;
					}
				}
				'<' =>
				{
					let next_non_ws = chars[index + 1..].iter().find(|c| !c.is_whitespace()).copied();
					if prev_non_ws.map(|c| c.is_alphanumeric() || c == '>' || c == '?').unwrap_or(false)
						&& next_non_ws.map(|c| c.is_alphanumeric() || c == '_' || c == '?' || c == '>').unwrap_or(false)
					{
						angle_depth += 1;
					}
				}
				'>' =>
				{
					if angle_depth > 0
					{
						angle_depth -= 1;
					}
				}
				'[' =>
				{
					if paren_depth == 0 && brace_depth == 0 && angle_depth == 0 && bracket_depth == 0
					{
						open_index = Some(index);
					}
					bracket_depth += 1;
				}
				']' =>
				{
					if bracket_depth > 0
					{
						bracket_depth -= 1;
						if bracket_depth == 0
						{
							if let Some(open) = open_index
							{
								return Some((open, index));
							}
						}
					}
				}
				_ => {}
			}

			if !ch.is_whitespace()
			{
				prev_non_ws = Some(ch);
			}

			index += 1;
		}

		return None;
	}

	fn paren_balance_delta(&self, line: &str) -> (i32, bool)
	{
		let chars: Vec<char> = line.chars().collect();
		let mut delta = 0;
		let mut has_open = false;
		let mut in_single_quote = false;
		let mut in_double_quote = false;

		for (index, ch) in chars.iter().enumerate()
		{
			match ch
			{
				'\'' =>
				{
					if !in_double_quote && !self.is_escaped(&chars, index)
					{
						in_single_quote = !in_single_quote;
					}
				}
				'"' =>
				{
					if !in_single_quote && !self.is_escaped(&chars, index)
					{
						in_double_quote = !in_double_quote;
					}
				}
				_ => {}
			}

			if in_single_quote || in_double_quote
			{
				continue;
			}

			match ch
			{
				'(' =>
				{
					delta += 1;
					has_open = true;
				}
				')' =>
				{
					delta -= 1;
				}
				_ => {}
			}
		}

		return (delta, has_open);
	}

	fn find_outermost_paren_indices(&self, chars: &[char]) -> Option<(usize, usize)>
	{
		let mut stack: Vec<usize> = Vec::new();
		let mut pairs: Vec<(usize, usize)> = Vec::new();

		let mut in_single_quote = false;
		let mut in_double_quote = false;

		for (index, ch) in chars.iter().enumerate()
		{
			match ch
			{
				'\'' =>
				{
					if !in_double_quote && !self.is_escaped(chars, index)
					{
						in_single_quote = !in_single_quote;
					}
				}
				'"' =>
				{
					if !in_single_quote && !self.is_escaped(chars, index)
					{
						in_double_quote = !in_double_quote;
					}
				}
				_ => {}
			}

			if in_single_quote || in_double_quote
			{
				continue;
			}

			match ch
			{
				'(' =>
				{
					stack.push(index);
				}
				')' =>
				{
					if let Some(open_index) = stack.pop()
					{
						pairs.push((open_index, index));
					}
				}
				_ => {}
			}
		}

		if pairs.is_empty()
		{
			return None;
		}

		return pairs.into_iter().max_by_key(|(_, close)| *close);
	}

	fn contains_trailing_comma(&self, before_closing: &str, arguments: &[String]) -> bool
	{
		if before_closing.trim_end().ends_with(',')
		{
			return true;
		}

		for argument in arguments
		{
			if self.expression_has_trailing_comma(argument)
			{
				return true;
			}
		}

		return false;
	}

	fn expression_has_trailing_comma(&self, argument: &str) -> bool
	{
		let chars: Vec<char> = argument.chars().collect();
		let mut stack: Vec<char> = Vec::new();
		let mut in_single_quote = false;
		let mut in_double_quote = false;
		let len = chars.len();
		let mut index = 0;

		while index < len
		{
			let ch = chars[index];

			match ch
			{
				'\'' =>
				{
					if !in_double_quote && !self.is_escaped(&chars, index)
					{
						in_single_quote = !in_single_quote;
					}
				}
				'"' =>
				{
					if !in_single_quote && !self.is_escaped(&chars, index)
					{
						in_double_quote = !in_double_quote;
					}
				}
				_ => {}
			}

			if in_single_quote || in_double_quote
			{
				index += 1;
				continue;
			}

			match ch
			{
				'(' | '[' | '{' =>
				{
					stack.push(ch);
				}
				')' =>
				{
					if let Some(open) = stack.pop()
					{
						if open == '(' && self.previous_non_whitespace_is_comma(&chars, index)
						{
							return true;
						}
					}
				}
				']' =>
				{
					if let Some(open) = stack.pop()
					{
						if open == '[' && self.previous_non_whitespace_is_comma(&chars, index)
						{
							return true;
						}
					}
				}
				'}' =>
				{
					if let Some(open) = stack.pop()
					{
						if open == '{' && self.previous_non_whitespace_is_comma(&chars, index)
						{
							return true;
						}
					}
				}
				_ => {}
			}

			index += 1;
		}

		return false;
	}

	fn previous_non_whitespace_is_comma(&self, chars: &[char], mut index: usize) -> bool
	{
		if index == 0
		{
			return false;
		}

		while index > 0
		{
			index -= 1;
			let ch = chars[index];

			if ch.is_whitespace()
			{
				continue;
			}

			return ch == ',';
		}

		return false;
	}

	fn is_escaped(&self, chars: &[char], index: usize) -> bool
	{
		if index == 0
		{
			return false;
		}

		let mut backslash_count = 0;
		let mut idx = index;

		while idx > 0
		{
			idx -= 1;
			if chars[idx] == '\\'
			{
				backslash_count += 1;
			}
			else
			{
				break;
			}
		}

		return backslash_count % 2 == 1;
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
