package fmt

import "io"

func Fprintf(writer io.Writer, format string, args ...any) (n int, err error)

/*
Sprintf: string(buffer)
Printf: Stdout
*/
