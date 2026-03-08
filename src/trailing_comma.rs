// Public entry point. Runs the single_pass loop until the output stabilises
// (up to 5 iterations). Multiple passes are needed so that nested structures
// converge to correct indentation: the first pass formats inner spans using
// the indentation context of the *original* source; subsequent passes
// re-read the now-correct indentation of each inner span and reformat
// accordingly.
pub(crate) fn format_trailing_commas(content: &str) -> String
{
	let mut result = content.to_string();
	for _ in 0..5
	{
		let next = single_pass(&result);
		if next == result
		{
			break;
		}
		result = next;
	}
	result
}

// ---------------------------------------------------------------------------
// Span — one matched delimiter pair
// ---------------------------------------------------------------------------

struct Span
{
	open: usize,         // byte index of opening delimiter  ( or [
	close: usize,        // byte index of closing delimiter  ) or ]
	indent_level: usize, // number of tabs at the start of the line that contains `open`
}

// ---------------------------------------------------------------------------
// single_pass — one full scan + replace cycle
// ---------------------------------------------------------------------------

fn single_pass(content: &str) -> String
{
	let mut spans = find_spans(content);
	if spans.is_empty()
	{
		return content.to_string();
	}

	// Process right-to-left (highest open index first = innermost/rightmost
	// first). This preserves byte offsets for outer spans that haven't been
	// processed yet — except that their `close` index shifts when the inner
	// replacement changes length, so we update those below.
	spans.sort_by(|a, b| b.open.cmp(&a.open));

	let mut result = content.to_string();

	for i in 0..spans.len()
	{
		let span_open = spans[i].open;
		let span_close = spans[i].close;
		let indent = spans[i].indent_level;

		let fragment = result[span_open..=span_close].to_string();
		let replacement = format_span(&fragment, indent);

		let old_len = span_close - span_open + 1;
		let new_len = replacement.len();

		result.replace_range(span_open..=span_close, &replacement);

		// Adjust the `close` index of any outer spans (lower open index,
		// therefore processed later) whose closing delimiter sits after the
		// region we just replaced.
		if new_len != old_len
		{
			let delta: isize = new_len as isize - old_len as isize;
			for j in (i + 1)..spans.len()
			{
				if spans[j].close > span_close
				{
					spans[j].close = (spans[j].close as isize + delta) as usize;
				}
			}
		}
	}

	result
}

// ---------------------------------------------------------------------------
// find_spans — character-level scanner
// ---------------------------------------------------------------------------

fn find_spans(content: &str) -> Vec<Span>
{
	let bytes = content.as_bytes();
	let n = bytes.len();

	let mut spans: Vec<Span> = Vec::new();
	// stack entries: (open_byte_pos, open_char, indent_level, should_record)
	// should_record is always true for ( and [.
	// For { it is true only when preceded by ( or , — i.e. a named parameter
	// block or a map/set literal passed as an argument, not a block body.
	let mut stack: Vec<(usize, char, usize, bool)> = Vec::new();

	// Tracks the last non-whitespace byte seen in Normal mode.
	// Used to decide whether { is a parameter block (preceded by ( or ,)
	// or a block body (preceded by ) or an identifier/keyword).
	let mut last_sig: u8 = b' ';

	let mut i = 0usize;
	let mut line_start = 0usize;

	// Scanner state
	let mut in_single_string = false;
	let mut single_quote: u8 = b'"';
	let mut in_multi_string = false;
	let mut multi_quote: u8 = b'"';
	let mut in_line_comment = false;

	while i < n
	{
		let c = bytes[i];

		// --- inside a line comment ---
		if in_line_comment
		{
			if c == b'\n'
			{
				in_line_comment = false;
				line_start = i + 1;
			}
			i += 1;
			continue;
		}

		// --- inside a multi-line string ---
		if in_multi_string
		{
			if c == b'\n'
			{
				line_start = i + 1;
			}
			else if c == multi_quote && i + 2 < n && bytes[i + 1] == multi_quote && bytes[i + 2] == multi_quote
			{
				in_multi_string = false;
				i += 3;
				continue;
			}
			i += 1;
			continue;
		}

		// --- inside a single-line string ---
		if in_single_string
		{
			if c == b'\\' && i + 1 < n
			{
				i += 2; // skip escape sequence
				continue;
			}
			if c == single_quote
			{
				in_single_string = false;
			}
			i += 1;
			continue;
		}

		// --- normal mode ---

		if c == b'\n'
		{
			line_start = i + 1;
			i += 1;
			continue;
		}

		// Check for multi-line string opening: ''' or """
		if (c == b'\'' || c == b'"') && i + 2 < n && bytes[i + 1] == c && bytes[i + 2] == c
		{
			in_multi_string = true;
			multi_quote = c;
			last_sig = c;
			i += 3;
			continue;
		}

		// Check for line comment //
		if c == b'/' && i + 1 < n && bytes[i + 1] == b'/'
		{
			in_line_comment = true;
			i += 2;
			continue;
		}

		// Check for single-line string opening
		if c == b'\'' || c == b'"'
		{
			in_single_string = true;
			single_quote = c;
			last_sig = c;
			i += 1;
			continue;
		}

		// Opening ( and [
		if c == b'(' || c == b'['
		{
			let indent = tabs_at_line_start(bytes, line_start);
			stack.push((i, c as char, indent, true));
			last_sig = c;
			i += 1;
			continue;
		}

		// Opening { — only record as a span when it is a named parameter block
		// or a map/set literal passed as an argument, identified by the
		// preceding non-whitespace character being ( or ,.
		// Block bodies are always preceded by ) (if-body, for-body, function
		// body after signature) or by an identifier/keyword (class, else, etc.)
		// and are therefore not recorded.
		if c == b'{'
		{
			let indent = tabs_at_line_start(bytes, line_start);
			let should_record = last_sig == b'(' || last_sig == b',';
			stack.push((i, '{', indent, should_record));
			last_sig = c;
			i += 1;
			continue;
		}

		// Closing delimiters
		if c == b')' || c == b']' || c == b'}'
		{
			if let Some(&(open_pos, open_char, indent, should_record)) = stack.last()
			{
				let matched = (c == b')' && open_char == '(')
					|| (c == b']' && open_char == '[')
					|| (c == b'}' && open_char == '{');

				if matched
				{
					stack.pop();
					if should_record
					{
						spans.push(Span { open: open_pos, close: i, indent_level: indent });
					}
				}
				// If not matched (malformed source) leave the stack as-is.
			}
			last_sig = c;
			i += 1;
			continue;
		}

		// All other Normal-mode characters — update last_sig for non-whitespace
		if c != b' ' && c != b'\t'
		{
			last_sig = c;
		}
		i += 1;
	}

	spans
}

// ---------------------------------------------------------------------------
// format_span — decide and render a single delimiter pair
// ---------------------------------------------------------------------------

fn format_span(fragment: &str, indent_level: usize) -> String
{
	if fragment.len() < 2
	{
		return fragment.to_string();
	}

	let open_char = fragment.chars().next().unwrap();
	let close_char = fragment.chars().last().unwrap();
	let inner = &fragment[1..fragment.len() - 1];

	let elements = split_elements(inner);
	let non_empty: Vec<String> = elements.into_iter().map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect();

	// Empty list — always single line, no change
	if non_empty.is_empty()
	{
		return format!("{}{}", open_char, close_char);
	}

	let trailing = has_trailing_comma(inner);
	let inline_comments = has_inline_comments(inner);

	if trailing || inline_comments
	{
		return render_multi_line(open_char, &non_empty, close_char, indent_level, trailing);
	}

	// No trailing comma and already single-line — leave completely untouched.
	if !fragment.contains('\n')
	{
		return fragment.to_string();
	}

	let single = render_single_line(open_char, &non_empty, close_char);

	// If the single-line form already contains newlines it means one of the
	// inner elements was expanded to multi-line by an earlier pass (e.g. a
	// named-parameter block { ... } that was just formatted). In that case,
	// leave the outer delimiter pair as-is — the natural wrapping produced by
	// the inner pass is already correct and adding another layer of indentation
	// would be wrong.
	if single.contains('\n')
	{
		return fragment.to_string();
	}

	// No trailing comma, was multi-line → collapse to single line.
	single
}

// ---------------------------------------------------------------------------
// split_elements — split inner content by top-level commas
// ---------------------------------------------------------------------------

fn split_elements(inner: &str) -> Vec<String>
{
	let mut elements: Vec<String> = Vec::new();
	let mut current = String::new();
	let mut depth = 0usize;

	let mut in_single = false;
	let mut single_q = '"';
	let mut in_multi = false;
	let mut multi_q = '"';
	let mut in_comment = false;

	let chars: Vec<char> = inner.chars().collect();
	let n = chars.len();
	let mut i = 0usize;

	while i < n
	{
		let c = chars[i];

		if in_comment
		{
			current.push(c);
			if c == '\n'
			{
				in_comment = false;
			}
			i += 1;
			continue;
		}

		if in_multi
		{
			current.push(c);
			if c == multi_q && i + 2 < n && chars[i + 1] == multi_q && chars[i + 2] == multi_q
			{
				current.push(chars[i + 1]);
				current.push(chars[i + 2]);
				in_multi = false;
				i += 3;
				continue;
			}
			i += 1;
			continue;
		}

		if in_single
		{
			current.push(c);
			if c == '\\' && i + 1 < n
			{
				current.push(chars[i + 1]);
				i += 2;
				continue;
			}
			if c == single_q
			{
				in_single = false;
			}
			i += 1;
			continue;
		}

		// Check for line comment
		if c == '/' && i + 1 < n && chars[i + 1] == '/'
		{
			in_comment = true;
			current.push(c);
			i += 1;
			continue;
		}

		// Check for multi-line string
		if (c == '\'' || c == '"') && i + 2 < n && chars[i + 1] == c && chars[i + 2] == c
		{
			in_multi = true;
			multi_q = c;
			current.push(c);
			i += 1;
			continue;
		}

		// Check for single-line string
		if c == '\'' || c == '"'
		{
			in_single = true;
			single_q = c;
			current.push(c);
			i += 1;
			continue;
		}

		// Track nesting
		if c == '(' || c == '[' || c == '{'
		{
			depth += 1;
			current.push(c);
			i += 1;
			continue;
		}
		if c == ')' || c == ']' || c == '}'
		{
			if depth > 0
			{
				depth -= 1;
			}
			current.push(c);
			i += 1;
			continue;
		}

		// Top-level comma = element separator
		if c == ',' && depth == 0
		{
			elements.push(current.clone());
			current = String::new();
			i += 1;
			continue;
		}

		current.push(c);
		i += 1;
	}

	// Push whatever remains after the last comma (may be empty for trailing comma)
	elements.push(current);

	elements
}

// ---------------------------------------------------------------------------
// has_trailing_comma
// ---------------------------------------------------------------------------

fn has_trailing_comma(inner: &str) -> bool
{
	// Walk lines in reverse; on the first non-blank line (after stripping any
	// inline comment) check whether the last meaningful char is a comma.
	for line in inner.lines().rev()
	{
		let effective = strip_line_comment(line.trim_end()).trim_end();
		if effective.is_empty()
		{
			continue;
		}
		return effective.ends_with(',');
	}

	// inner had no newlines — treat it as a single line
	let effective = strip_line_comment(inner.trim_end()).trim_end();
	effective.ends_with(',')
}

fn strip_line_comment(s: &str) -> &str
{
	// Simplified: find the first `//` not inside a string.
	// For correctness we do a minimal scan rather than just `find("//")`.
	let chars: Vec<char> = s.chars().collect();
	let n = chars.len();
	let mut i = 0usize;
	let mut in_str = false;
	let mut str_char = '"';
	while i < n
	{
		let c = chars[i];
		if in_str
		{
			if c == '\\' && i + 1 < n
			{
				i += 2;
				continue;
			}
			if c == str_char
			{
				in_str = false;
			}
		}
		else
		{
			if c == '"' || c == '\''
			{
				in_str = true;
				str_char = c;
			}
			else if c == '/' && i + 1 < n && chars[i + 1] == '/'
			{
				// Return the slice up to this point
				let byte_pos = s.char_indices().nth(i).map(|(b, _)| b).unwrap_or(s.len());
				return &s[..byte_pos];
			}
		}
		i += 1;
	}
	s
}

// ---------------------------------------------------------------------------
// has_inline_comments
// ---------------------------------------------------------------------------

fn has_inline_comments(inner: &str) -> bool
{
	for line in inner.lines()
	{
		let trimmed = line.trim();
		if trimmed.starts_with("//")
		{
			// Full-line comment — does not force multi-line on its own
			continue;
		}
		// Inline comment: there is text before the `//`
		if strip_line_comment(trimmed).trim_end().len() < trimmed.len()
		{
			return true;
		}
	}
	false
}

// ---------------------------------------------------------------------------
// Renderers
// ---------------------------------------------------------------------------

fn render_single_line(open: char, elements: &[String], close: char) -> String
{
	let parts: Vec<&str> = elements.iter().map(|e| e.trim()).collect();
	format!("{}{}{}", open, parts.join(", "), close)
}

fn render_multi_line(open: char, elements: &[String], close: char, indent_level: usize, has_trailing: bool) -> String
{
	let indent = "\t".repeat(indent_level);
	let child_indent = "\t".repeat(indent_level + 1);

	let mut out = format!("{}\n", open);

	for (idx, element) in elements.iter().enumerate()
	{
		let is_last = idx == elements.len() - 1;
		let trimmed = element.trim();
		if is_last && !has_trailing
		{
			out.push_str(&format!("{}{}\n", child_indent, trimmed));
		}
		else
		{
			out.push_str(&format!("{}{},\n", child_indent, trimmed));
		}
	}

	out.push_str(&format!("{}{}", indent, close));
	out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tabs_at_line_start(bytes: &[u8], line_start: usize) -> usize
{
	let mut count = 0usize;
	let mut i = line_start;
	while i < bytes.len() && bytes[i] == b'\t'
	{
		count += 1;
		i += 1;
	}
	count
}

