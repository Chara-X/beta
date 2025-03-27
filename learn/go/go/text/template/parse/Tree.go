package parse

import "text/template/parse"

type Tree struct {
	Name string
	Root parse.Node
}

func (t *Tree) Parse(text, leftDelim, rightDelim string, trees map[string]*Tree, funcs ...map[string]any) error

/*
error: (tree *Tree, err error)
*/
