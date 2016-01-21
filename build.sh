#!/bin/sh

echo Running...
rustdoc slides.md -o . --html-in-header=header.inc.html --markdown-no-toc
rustdoc index.md -o . --markdown-no-toc
