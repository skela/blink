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

use tree_sitter::{Language, Node, Parser};

extern "C"
{
	fn tree_sitter_dart() -> Language;
}

impl Formatter
{
	fn analyze(&self, source_code: &String)
	{
		let mut parser = Parser::new();

		let language = unsafe { tree_sitter_dart() };
		parser.set_language(language).unwrap();

		let tree = parser.parse(source_code, None).unwrap();
		let root_node = tree.root_node();

		self.analyze_node(root_node, 0);
	}

	fn analyze_node(&self, node: Node, level: usize)
	{
		let mut cursor = node.walk();
		let children = node.children(&mut cursor);

		for child in children
		{
			println!("{}{}", "\t".repeat(level), child.kind());
			self.analyze_node(child, level + 1);
		}
	}

	fn tree_sitter_sample(&self) -> String
	{
		let src = r#"
class ABC
{
	int b = 2;
}

class DEF
{
    final int a;
    final int b;

    DEF({required this.a,required this.b});
}

		class Testing {
			Testing();

			int a = 1;

			void testingLoops(){
				int sum = 0;
				for (int i = 0; i<5; i++)
					sum += 1;
			}

			void testingLoopsWithCurlies(){
				int sum = 0;
				for (int i = 0; i<5; i++){
					sum += 1;
				}
			}

			bool testingReturn(){
				return true;
			}

			bool get testingInlineReturn => true;

			Map<String,dynamic> dict = {
				"testing":1,
			};

            void testingSwitchBreaks()
            {
                final animal = Animal.Dog;
                switch(animal){
                    case Animal.Dog: print("This is a dog"); break;
                    case Animal.Cat:
                        print("This is a cat");
                    break;
                    case Animal.Horse:{
                        print("This is a horse");
							print("which is pretty cool");
                    }
                    break;
                    case Animal.Bird: print("This is a bird"); break;
                }
            }
		}

enum Animal{
    Dog,
		Cat,
    Horse,
    Bird,
}

		"#;

		return src.to_string();
	}

	pub(crate) fn tree_sitter_analyze(&self)
	{
		let src = self.tree_sitter_sample();
		self.analyze(&src);
	}
	
	fn indent_symbol(&self) -> char { return '\t'; }

	pub(crate) fn tree_sitter_format(&self)
	{
		let mut src = self.tree_sitter_sample();

		let mut parser = Parser::new();

		let language = unsafe { tree_sitter_dart() };
		parser.set_language(language).unwrap();

		let mut tree = parser.parse(&src, None).unwrap();
		let mut root_node = tree.root_node();

		let indents = self.locate_indents(&mut src, root_node, 0);

		let indent_symbol = self.indent_symbol().to_string();

		for indent in indents.iter().rev()
		{
			src.replace_range(indent.start..indent.end, indent_symbol.repeat(indent.indent).as_str());
		}

		tree = parser.parse(&src, None).unwrap();
		root_node = tree.root_node();

		let curlies = self.locate_curlies(&mut src, root_node, 0);

		for curly in curlies.iter().rev()
		{
			if curly.inject_newline
			{
				src.insert(curly.location, '\n');
				for i in 0..curly.indent
				{
					src.insert(curly.location + i + 1, self.indent_symbol());
				}
			}
			else
			{
				if let Some(lstart) = src[..curly.location-1].rfind('\n')
				{
					src.replace_range(lstart+1..curly.location,self.indent_symbol().to_string().repeat(curly.indent).as_str());
				}
			}
		}
		println!("String is {}", src);
	}

	fn find_curly_parent_coordinates(&self, string: &mut String,node: Node) -> Option<FormatNodeCoordinates>
	{
		if let Some(p) = node.parent()
		{
			return match p.kind()
			{
				"set_or_map_literal" => None,
				"switch_block" =>
				{
					if let Some(sw) = string[..p.start_byte()].rfind("switch")
					{
						return Some(FormatNodeCoordinates { start_byte: sw, kind: p.kind().to_string() });
					}
					return None
				}
				"class_body" | "enum_body" =>
				{
					if let Some(p2) = p.parent()
					{
						return Some(p2.format_coordinates())
					}
					return Some(p.format_coordinates())
				}
				"block" =>
				{
					if let Some(p2) = p.parent()
					{
						if p2.kind() == "switch_statement_case"
						{
							return Some(p2.format_coordinates())
						}
						else if p2.kind() == "function_body"
						{
							if let Some(ps) = p2.prev_sibling()
							{
								return Some(ps.format_coordinates())
							}
							return None
						}
						else if p2.kind() == "for_statement"
						{
							return Some(p2.format_coordinates())
						}
					}
					return None
				}
				_ => None,
			};
		}
		return None;
	}

	fn locate_curlies(&self, string: &mut String, node: Node, level: usize) -> Vec<FormatCurly>
	{
		let mut curlies: Vec<FormatCurly> = Vec::new();
		let mut cursor = node.walk();
		let children = node.children(&mut cursor);

		for child in children
		{
			if child.kind().eq("{")
			{
				if let Some(parent) = self.find_curly_parent_coordinates(string,child)
				{
					let pstart = parent.start_byte;
					let c = child.end_byte();
					let sub = string.substring(pstart, c);
					if !sub.contains("\n")
					{
						let mut indent : usize = 0;
						if let Some(lstart) = string[..child.start_byte()].rfind('\n')
						{
							for char in string[lstart+1..child.start_byte()].chars()
							{
								if char == self.indent_symbol() { indent += 1; }
								else { break; }
							}
						}
						curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: true });
					}
					else
					{
						let mut indent : usize = 0;
						if let Some(lstart) = string[..pstart].rfind('\n')
						{
							let sub2 = string.substring(lstart+1,pstart);
							if sub2.is_empty() { continue }
							for char in sub2.chars()
							{
								if char == self.indent_symbol() { indent += 1; }
								else { break; }
							}
							curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: false });
						}
					}
				}
			}
			if child.kind().eq("}")
			{
				if let Some(parent) = self.find_curly_parent_coordinates(string,child)
				{
					let pstart = parent.start_byte;

					let mut indent : usize = 0;
					if let Some(lstart) = string[..pstart].rfind('\n')
					{
						let sub = string.substring(lstart+1,pstart);
						if sub.is_empty() { continue }
						for char in sub.chars()
						{
							if char == self.indent_symbol() { indent += 1; }
							else { break; }
						}
						curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: false });
					}
				}
			}
			curlies.extend(self.locate_curlies(string, child, level + 1));
		}
		return curlies;
	}

	fn indent_from_level(&self, node: Node, level: usize) -> usize
	{
		return match node.kind()
		{
			"class_definition" | "enum_declaration" => level,
			"declaration" | "method_signature" => level - 1,
			"enum_constant" => level - 1,
			"local_variable_declaration" => level - 2,
			"return_statement" => level - 2,
			"for_statement" => level - 2,
			"switch_statement" => level - 2,
			"switch_statement_case" => level - 3,
			"break_statement" => level - 4,
			"expression_statement" => 
			{
				if node.parent_kind() == "switch_statement_case" { return level - 3 }
				else if node.parent_kind() == "block" { return level - 4 }
				return level - 3
			}
			_ => level,
		};
	}

	fn locate_indents(&self, string: &mut String, node: Node, level: usize) -> Vec<FormatIndent>
	{
		let mut indents: Vec<FormatIndent> = Vec::new();
		let mut cursor = node.walk();
		let children = node.children(&mut cursor);

		for child in children
		{
			match child.kind()
			{
				"class_definition" | "enum_declaration" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let end = cstart;
						// println!("found class/enum at {} - lstart {} - indent {}:\n{}", end, start, self.indent_from_level(child, level), string.substring(start, end + 11));
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"}" =>
				{
					if child.parent_kind() == "class_body" || child.parent_kind() == "enum_body"
					{
						let cstart = child.start_byte();
						if let Some(lstart) = string[..cstart].rfind('\n')
						{
							let start = lstart + 1;
							let end = cstart;
							if start != end
							{
								indents.push(FormatIndent { start, end, indent: level - 2 });
							}
						}
					}
				}
				"method_signature" | "declaration" | "enum_constant" | "switch_statement" | "return_statement" | "for_statement" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let end = cstart;
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"switch_statement_case" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let end = cstart;
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"break_statement" | "expression_statement" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						if string.substring(lstart,cstart).contains(";") || string.substring(lstart,cstart).contains(":") { continue; }
						let start = lstart + 1;
						let end = cstart;
						// println!("found break statement at {} - lstart {} - indent {}:\n{}", end, start, self.indent_from_level(child, level), string.substring(start, end + 11));
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"local_variable_declaration" =>
				{
					if child.parent_kind() != "block" { continue; }
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let end = cstart;
						// println!("found local variable at {} - lstart {} - indent {}:\n{}", end, start, self.indent_from_level(child, level), string.substring(start, end + 11));
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				_ => (),
			}
			indents.extend(self.locate_indents(string, child, level + 1));
		}
		return indents;
	}
}

struct FormatIndent
{
	start: usize,
	end: usize,
	indent: usize,
}

struct FormatCurly
{
	location: usize,
	indent: usize,
	inject_newline: bool
}

trait FormatCoordinator
{
	fn format_coordinates(&self) -> FormatNodeCoordinates;
	fn parent_kind(&self) -> &str;
}

impl FormatCoordinator for Node<'_>
{
	fn format_coordinates(&self) -> FormatNodeCoordinates
	{
		return FormatNodeCoordinates { start_byte: self.start_byte(), kind: self.kind().to_string() };
	}

	fn parent_kind(&self) -> &str
	{
		if let Some(p) = self.parent()
		{
			return p.kind();
		}
		return "";
	}
}

struct FormatNodeCoordinates
{
	start_byte: usize,
	kind: String,
}
