default: build run

.PHONY: build
build:
	dart compile exe bin/format.dart -o bin/sdartfmt

.PHONY: run
run:
	bin/sdartfmt -w benchmark/testrun.dart
