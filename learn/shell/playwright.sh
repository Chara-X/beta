#!/usr/bin/bash

playwright install --with-deps
playwright codegen --save-storage filename --load-storage filename
