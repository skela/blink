default: build run

.PHONY: build
build:
	dart compile exe bin/format.dart -o bin/sdartfmt

.PHONY: format
format:
	bin/sdartfmt --fix -w benchmark/testrun.dart

.PHONY: astyle
astyle:
	astyle --style=allman \
	--indent=tab \
	--keep-one-line-blocks \
	--keep-one-line-statements \
	benchmark/testrun.dart

.PHONY: both
both: format astyle

.PHONY: reset
reset:
	git checkout benchmark/testrun.dart

# --unpad-paren \