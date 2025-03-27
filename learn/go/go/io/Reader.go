package io

type Reader interface {
	Read(p []byte) (n int, err error)
}

func LimitReader(r Reader, n int64) Reader
func MultiReader(readers ...Reader) Reader
