#!/usr/bin/bash

playwright install
playwright uninstall
playwright codegen --save-storage filename --load-storage filename
