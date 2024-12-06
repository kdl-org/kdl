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

if orphanedFiles:
	SUCCESS = False
	print("ERROR: There are outputs in /expected_kdl without corresponding tests in /input:\n" + "\n".join(["  "+x for x in orphanedFiles]))

misnamedFiles: list[str] = []
for filepath in invalidFiles:
	basepath, ext = os.path.splitext(filepath)
	if not basepath.endswith("_fail"):
		misnamedFiles.append(filepath)
if misnamedFiles:
	SUCCESS = False
	print("ERROR: There are tests in /input without corresponding outputs in /expected_kdl, but they don't have a _fail suffix:\n" + "\n".join(["  "+x for x in misnamedFiles]))

if not SUCCESS:
	sys.exit(1)