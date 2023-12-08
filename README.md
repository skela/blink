## Blink - A Blazing Fast and Sensible Dart Code Formatter (WIP)

Goal is to build a simple replacement to the absolutely bonkers and utter crazy opinionated formatter for dart written by Google and friends, which unfortunatealy seems to be the only available formatter for dart that I've come across.

We may not need or even want to do all the crazy things the dartfm does, but here's some goals that I would like to achieve with this project:

* Write it in Rust, because Rust is awesome.
* Make use of editorconfig files, most sensible projects have them defined (or should).
* Fix up incorrectly placed curly braces (if editorconfig has curly_brace_on_next_line=true)
* Fix up incorrect indentation (if editorconfig has indent_style = tab)
* Fix up incorrect single quote usage (if editorconfig has prefer_double_quotes=true)

Sample of .editorconfig that can be used:

```
[*.dart]
indent_size = 2
curly_brace_on_next_line = true
prefer_double_quotes = true
```

# Getting started

You will need to install rust in order to build this project.

For Linux (Arch): `yay install rust`

For macOS: `brew install rust`

# Treesitter branch
- `git clone git@github.com:UserNobody14/tree-sitter-dart.git`
