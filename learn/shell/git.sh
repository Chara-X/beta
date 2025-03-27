#!/usr/bin/bash

git init
git clone repository --depth 1
git add -A
git commit -m message --amend
git reset commit
git push repository HEAD:branch
git pull repository branch
