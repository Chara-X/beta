package termui

import (
	"image"
	"sync"

	"github.com/gizak/termui/v3"
)

type Block struct {
	self *termui.Block
	image.Rectangle
	sync.Mutex
	Inner                                                image.Rectangle
	Title                                                string
	TitleStyle                                           termui.Style
	Border                                               bool
	BorderStyle                                          termui.Style
	BorderLeft, BorderRight, BorderTop, BorderBottom     bool
	PaddingLeft, PaddingRight, PaddingTop, PaddingBottom int
}

func NewBlock() *Block                         { return &Block{self: termui.NewBlock()} }
func (self *Block) SetRect(x1, y1, x2, y2 int) { self.self.SetRect(x1, y1, x2, y2) }
func (self *Block) GetRect() image.Rectangle   { return self.self.GetRect() }
func (self *Block) Draw(buf *termui.Buffer)    { self.self.Draw(buf) }
