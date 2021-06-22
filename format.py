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
	
	params = [		
		"indent_with_tabs=2", # 1=indent to level only, 2=indent with tabs
	]
	
	cmd = "uncrustify -h"
	os.system(cmd)

if m == "astyle":
	astyle()
if m == "uncrustify":
	uncrustify()
if m == "sdartfmt":
	sdartfmt()
