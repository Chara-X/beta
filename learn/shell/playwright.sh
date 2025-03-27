#!/usr/bin/bash

playwright install browser --dry-run --with-deps --only-shell --no-shell
playwright uninstall --all
playwright codegen --save-storage filename --load-storage filename
