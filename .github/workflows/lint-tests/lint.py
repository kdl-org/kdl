from __future__ import annotations

import os
import sys
import typing

def findTestFiles(path) -> typing.Generator[str, None, None]:
    for root, _, filenames in os.walk(path):
        for filename in filenames:
            yield os.path.join(root, filename)

# strip the leading folder name, so they can be directly compared
inputFiles = set(x[len("input")+1:] for x in findTestFiles("input"))
validFiles = set(x[len("expected_kdl")+1:] for x in findTestFiles("expected_kdl"))

invalidFiles = inputFiles - validFiles
orphanedFiles = validFiles - inputFiles

SUCCESS = True

# Check for any expected_kdl files without a corresponding input file.
if orphanedFiles:
	SUCCESS = False
	print("ERROR: There are outputs in /expected_kdl without corresponding tests in /input:\n" + "\n".join(["  "+x for x in orphanedFiles]))

# Check for any input files lacking an expected_kdl file
# (aka inputs expected to generate a parse error)
# that don't have a _fail suffix.
misnamedFiles: list[str] = []
for filepath in invalidFiles:
	basepath, ext = os.path.splitext(filepath)
	if not basepath.endswith("_fail"):
		misnamedFiles.append(filepath)
if misnamedFiles:
	SUCCESS = False
	print("ERROR: There are tests in /input without corresponding outputs in /expected_kdl, but they don't have a _fail suffix:\n" + "\n".join(["  "+x for x in misnamedFiles]))

# Check for any expected_kdl files that don't end in a newline.
noNewlineFiles: list[str] = []
for filepath in validFiles:
	with open("expected_kdl/" + filepath, "r", encoding="utf-8") as fh:
		text = fh.read()
		if not text.endswith("\n"):
			noNewlineFiles.append(filepath)
if noNewlineFiles:
	SUCCESS = False
	print("ERROR: There are outputs in /expected_kdl that don't end with a newline:\n" + "\n".join(["  "+x for x in noNewlineFiles]))

if not SUCCESS:
	sys.exit(1)