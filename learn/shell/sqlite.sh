#!/usr/bin/bash

sqlite3
.open file --deserialize
.mode mode
.output file
.help --all
sql

# sqlite_schema
