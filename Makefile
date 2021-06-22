default: build run

.PHONY: build
build:
	dart compile exe bin/format.dart -o bin/sdartfmt

.PHONY: run
run:
	bin/sdartfmt --fix -w benchmark/testrun.dart

.PHONY: arun
arun:
	astyle --style=allman \
	--indent=tab \
	--keep-one-line-blocks \
	--keep-one-line-statements \
	benchmark/testrun.dart

.PHONY: both
both: run arun

.PHONY: reset
reset:
	git checkout benchmark/testrun.dart

# --unpad-paren \