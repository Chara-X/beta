// # Headings
//
// Paragraphs.
//
//	Code blocks
//
// [Doc.links]
//
//   - Lists
//
// [Links]
//
// [Links]: http://url
package godoc

type Doc struct{}

func (d *Doc) links()
func init() {
	var d = &Doc{}
	d.links()
}
