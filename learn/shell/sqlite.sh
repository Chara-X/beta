#!/usr/bin/bash

sqlite3 databasefile \
    .tables \
    .schema table \
    .indexes table \
    .header on/off \
    .mode mode \
    .import file table \
    .read file \
    .dump table \
    .help --all \
    sql
