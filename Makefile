default: build run

.PHONY: build
build:
	dart compile exe bin/format.dart -o bin/sdartfmt

.PHONY: reset
reset:
	cp benchmark/test1.dart.txt benchmark/test1.dart

.PHONY: sdartfmt
sdartfmt: reset _format_with_sdartfmt

.PHONY: astyle
astyle: reset _format_with_astyle

.PHONY: both
both: reset _format_with_sdartfmt _format_with_astyle

.PHONY: _format_with_sdartfmt
_format_with_sdartfmt:
	bin/sdartfmt --fix -w benchmark/test1.dart

.PHONY: _format_with_astyle
_format_with_astyle:
	astyle --style=allman \
	--indent=tab \
	--keep-one-line-blocks \
	--keep-one-line-statements \
	benchmark/test1.dart
