package fmt

import "io"

func Fscanf(reader io.Reader, format string, args ...any) (n int, err error)

/*
Sscanf: stringReader
Scanf: Stdin
*/
