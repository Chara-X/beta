package template

import (
	"io"

	"github.com/Chara-X/go-study/go/text/template/parse"
)

type Template struct{ *parse.Tree }

func New(name string) *Template
func (t *Template) Templates() []*Template
func (t *Template) New(name string) *Template
func (t *Template) Funcs(funcs map[string]any) *Template
func (t *Template) Parse(text string) error
func (t *Template) Execute(wr io.Writer, data any) error

/*
error: (*Template, error)
*/
