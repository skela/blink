import os
import argparse
import json
import subprocess
import fileinput

test_file = "benchmark/test1.dart"

parser = argparse.ArgumentParser()
parser.add_argument("--method","-m", help="Method, either astyle,uncrustify or sdartfmt")
args = parser.parse_args()

m = args.method

def sdartfmt():
	cmd = "bin/sdartfmt --fix -w benchmark/test1.dart"
	os.system(cmd)

def astyle():

	params = [
		"--style=allman",
		"--indent=tab",
		"--keep-one-line-blocks",
		"--keep-one-line-statements",
		"--indent-continuation=1",
		"--suffix=none",
		"--mode=cs",
	]

	all = " ".join(params)
	cmd = f"astyle {all} {test_file}"
	os.system(cmd)

def uncrustify():		
	cmd = "uncrustify -c uncrustify_style.cfg -f benchmark/test1.dart -o benchmark/test1-uncrust.dart ; diff benchmark/test1.dart benchmark/test1-uncrust.dart"
	os.system(cmd)

def codebuff():
	codebuff = "java -jar codebuff/target/codebuff-1.5.1.jar"
	cmd = f"{codebuff}"
	os.system(cmd)

def blink():
	cmd = "cargo run --manifest-path blink/Cargo.toml -- benchmark/test1.dart --output results"
	os.system(cmd)

if m == "astyle":
	astyle()
if m == "uncrustify":
	uncrustify()
if m == "sdartfmt":
	sdartfmt()
if m == "codebuff":
	codebuff()
if m == "blink":
	blink()
