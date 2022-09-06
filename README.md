## Blink - A Sensible Dart Code Formatter (WIP)

Goal is to build a simple replacement to the absolutely bonkers and utter crazy opinionated formatter for dart written by Google and friends, which unfortunatealy seems to be the only available formatter for dart that I've come across.

We may not need or even want to do all the crazy things the dartfm does, but here's some goals that I would like to achieve with this project:

* Make use of editorconfig files.
* Fix up incorrectly placed curly braces (if editorconfig has curly_brace_on_next_line=true)
* Fix up incorrect indentation (if editorconfig has indent_style = tab)
