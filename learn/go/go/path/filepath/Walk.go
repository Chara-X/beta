package filepath

import "io/fs"

func Walk(root string, fn func(path string, info fs.FileInfo, err error) error) error
