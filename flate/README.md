# flate

## Example

```go
func ExampleDeflate() {
	var input = bytes.NewBuffer(nil)
	var w = flate.NewDeflator(input)
	w.WriteHeader()
	w.Write([]byte("abc"))
	w.WriteTailer()
	w.Close()
	fmt.Println("src:", len(input.String()), input.String())
	var r = gzip.NewReader(input)
	defer r.Close()
	var output = bytes.NewBuffer(nil)
	io.Copy(output, r)
	fmt.Println("dst:", len(output.String()), output.String())
}
```

## Lucky case

abc*5

## Reference

[DEFLATE Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1951)

[Data Compression (Summer 2020) - Lecture 11 - DEFLATE (gzip)](https://www.youtube.com/watch?v=oi2lMBBjQ8s)

[compress/flate](https://pkg.go.dev/compress/flate)

[huffman coding in deflate](https://www.mrbluyee.com/2024/03/28/huffman-coding-in-deflate/)

[Compression algorithm (deflate)](https://raw.githubusercontent.com/madler/zlib/master/doc/algorithm.txt)
