use substring::Substring;

use tree_sitter::{Language, Node, Parser};

use crate::formatter::Formatter;

extern "C" {
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
				"yay":true,
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

  void testingIfs() {
  final animal = Animal.Dog;
  if (animal == Animal.Dog)
  print("this is a dog");
  else
  print("this is some other kind of animal");
  }

			void testingIfsWithCurlies(){
		final animal = Animal.Dog;
		if (animal == Animal.Dog){
		print("this is a dog");
		}
		else if (animal == Animal.Cat) {
		print("this is a cat");
		}
		else{
	print("this is some other kind of animal");
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

	fn indent_symbol(&self) -> char
	{
		return '\t';
	}

	pub(crate) fn format_using_treesitter(&self, code: String) -> String
	{
		let mut src = String::from(code);

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

		// tree = parser.parse(&src, None).unwrap();
		// root_node = tree.root_node();

		// let curlies = self.locate_curlies(&mut src, root_node, 0);
		//
		// for curly in curlies.iter().rev()
		// {
		// 	if curly.inject_newline
		// 	{
		// 		src.insert(curly.location, '\n');
		// 		for i in 0..curly.indent
		// 		{
		// 			src.insert(curly.location + i + 1, self.indent_symbol());
		// 		}
		// 	}
		// 	else
		// 	{
		// 		if let Some(lstart) = src[..curly.location - 1].rfind('\n')
		// 		{
		// 			src.replace_range(lstart + 1..curly.location, self.indent_symbol().to_string().repeat(curly.indent).as_str());
		// 		}
		// 	}
		// }

		return src;
	}

	pub(crate) fn tree_sitter_format(&self)
	{
		let src = self.format_using_treesitter(self.tree_sitter_sample());

		println!("String is {}", src);
	}

	fn find_curly_parent_coordinates(&self, string: &mut String, node: Node) -> Option<FormatNodeCoordinates>
	{
		if let Some(p) = node.parent()
		{
			return match p.kind()
			{
				"set_or_map_literal" =>
				{
					// if u want {} new lines leave this in
					// return Some(p.format_coordinates())
					if node.kind() == "}"
					{
						return Some(p.format_coordinates());
					}
					return None;
				}
				"switch_block" =>
				{
					if let Some(sw) = string[..p.start_byte()].rfind("switch")
					{
						return Some(FormatNodeCoordinates { start_byte: sw });
					}
					return None;
				}
				"class_body" | "enum_body" =>
				{
					if let Some(p2) = p.parent()
					{
						return Some(p2.format_coordinates());
					}
					return Some(p.format_coordinates());
				}
				"block" =>
				{
					if let Some(p2) = p.parent()
					{
						if p2.kind() == "switch_statement_case"
						{
							return Some(p2.format_coordinates());
						}
						else if p2.kind() == "function_body"
						{
							if let Some(ps) = p2.prev_sibling()
							{
								return Some(ps.format_coordinates());
							}
							return None;
						}
						else if p2.kind() == "for_statement"
						{
							return Some(p2.format_coordinates());
						}
						else if p2.kind() == "if_statement"
						{
							if let Some(psib) = p.prev_sibling()
							{
								if psib.kind() == "else"
								{
									return Some(FormatNodeCoordinates { start_byte: psib.start_byte() });
								}
							}
							if let Some(psib) = p2.prev_sibling()
							{
								if psib.kind() == "else"
								{
									return Some(FormatNodeCoordinates { start_byte: psib.start_byte() });
								}
							}
							return Some(p2.format_coordinates());
						}
					}
					return None;
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
				if let Some(parent) = self.find_curly_parent_coordinates(string, child)
				{
					let pstart = parent.start_byte;
					let c = child.end_byte();
					let sub = string.substring(pstart, c);
					if !sub.contains("\n")
					{
						let mut indent: usize = 0;
						if let Some(lstart) = string[..child.start_byte()].rfind('\n')
						{
							for char in string[lstart + 1..child.start_byte()].chars()
							{
								if char == self.indent_symbol()
								{
									indent += 1;
								}
								else
								{
									break;
								}
							}
						}
						curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: true });
					}
					else
					{
						let mut indent: usize = 0;
						if let Some(lstart) = string[..pstart].rfind('\n')
						{
							let sub2 = string.substring(lstart + 1, pstart);
							if sub2.is_empty()
							{
								continue;
							}
							for char in sub2.chars()
							{
								if char == self.indent_symbol()
								{
									indent += 1;
								}
								else
								{
									break;
								}
							}
							curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: false });
						}
					}
				}
			}
			if child.kind().eq("}")
			{
				if let Some(parent) = self.find_curly_parent_coordinates(string, child)
				{
					let pstart = parent.start_byte;

					let mut indent: usize = 0;
					if let Some(lstart) = string[..pstart].rfind('\n')
					{
						let sub = string.substring(lstart + 1, pstart);
						if sub.is_empty()
						{
							continue;
						}
						for char in sub.chars()
						{
							if char == self.indent_symbol()
							{
								indent += 1;
							}
							else
							{
								break;
							}
						}
						curlies.push(FormatCurly { location: child.start_byte(), indent, inject_newline: false });
					}
				}
			}
			curlies.extend(self.locate_curlies(string, child, level + 1));
		}
		return curlies;
	}

	fn indents_from_parent(&self, node: Node, indent: usize) -> usize
	{
		if let Some(p) = node.parent()
		{
			return match p.kind()
			{
				"block" => self.indents_from_parent(p, indent),
				"class_body" => self.indents_from_parent(p, indent),
				"function_body" => self.indents_from_parent(p, indent),
				"switch_block" => self.indents_from_parent(p, indent),
				"set_or_map_literal" => self.indents_from_parent(p, indent),
				"initialized_identifier_list" => self.indents_from_parent(p, indent),
				"initialized_identifier" => self.indents_from_parent(p, indent),
				"declaration" => self.indents_from_parent(p, indent),
				"pair" => self.indents_from_parent(p, indent),
				"if_statement" => self.indents_from_parent(p, indent),
				_ => self.indents_from_parent(p, indent + 1),
			};
		}
		return indent;
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
			// "if_statement" => level - 2,
			// "if" => level - 3,
			// "else" => level - 3,
			"if_statement" => self.indents_from_parent(node, 0),
			"if" => self.indents_from_parent(node, 0),
			"else" => self.indents_from_parent(node, 0),
			"break_statement" => level - 4,
			"expression_statement" =>
			{
				let indent = self.indents_from_parent(node, 0);
				if node.parent_kind() == "if_statement" || node.grand_parent_kind() == "if_statement"
				{
					return indent + 1;
				}
				return indent;
			}
			"pair" => self.indents_from_parent(node, 0),
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
				"if" | "else" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let mut end = cstart;
						if let Some(t) = string[..cstart].rfind("else")
						{
							if t > start
							{
								end = t;
							}
						}
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
						if string.substring(lstart, cstart).contains(";") || string.substring(lstart, cstart).contains(":")
						{
							continue;
						}
						let start = lstart + 1;
						let end = cstart;
						// println!("found break statement at {} - lstart {} - indent {}:\n{}", end, start, self.indent_from_level(child, level), string.substring(start, end + 11));
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"pair" =>
				{
					let cstart = child.start_byte();
					if let Some(lstart) = string[..cstart].rfind('\n')
					{
						let start = lstart + 1;
						let end = cstart;
						// println!("found pair statement at {} - lstart {} - indent {}:\n{}", end, start, self.indent_from_level(child, level), string.substring(start, end + 11));
						if start != end
						{
							indents.push(FormatIndent { start, end, indent: self.indent_from_level(child, level) });
						}
					}
				}
				"local_variable_declaration" =>
				{
					if child.parent_kind() != "block"
					{
						continue;
					}
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
	inject_newline: bool,
}

trait FormatCoordinator
{
	fn format_coordinates(&self) -> FormatNodeCoordinates;
	fn parent_kind(&self) -> &str;
	fn grand_parent_kind(&self) -> &str;
	fn line(&self, string: &mut String) -> String;
}

impl FormatCoordinator for Node<'_>
{
	fn format_coordinates(&self) -> FormatNodeCoordinates
	{
		return FormatNodeCoordinates { start_byte: self.start_byte() };
	}

	fn parent_kind(&self) -> &str
	{
		if let Some(p) = self.parent()
		{
			return p.kind();
		}
		return "";
	}

	fn grand_parent_kind(&self) -> &str
	{
		if let Some(p) = self.parent()
		{
			if let Some(gp) = p.parent()
			{
				return gp.kind();
			}
		}
		return "";
	}

	fn line(&self, string: &mut String) -> String
	{
		if let Some(left) = string[..self.start_byte()].rfind('\n')
		{
			if let Some(right) = string[self.start_byte()..].find('\n')
			{
				return string.substring(left, right).to_string();
			}
		}
		return "".to_string();
	}
}

struct FormatNodeCoordinates
{
	start_byte: usize,
}
