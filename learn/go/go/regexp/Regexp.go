package regexp

type Regexp struct{}

func Compile(expr string) (*Regexp, error)
func (re *Regexp) Match(b []byte) bool
func (re *Regexp) FindAll(b []byte, n int) [][]byte
func (re *Regexp) ReplaceAll(src, repl []byte) []byte
func (re *Regexp) Split(s string, n int) []string
func (re *Regexp) String() string

/*
func (re *Regexp) doExecute(r io.RuneReader, b []byte, s string, pos int, ncap int, dstCap []int)
`ReplaceFunc` can be implemented by `ReplaceAll`'s `Expand`
*/
