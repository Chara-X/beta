package bolt

type Cursor struct{}

func (c *Cursor) First() (key []byte, value []byte)
func (c *Cursor) Last() (key []byte, value []byte)
func (c *Cursor) Seek(seek []byte) (key []byte, value []byte)
func (c *Cursor) Prev() (key []byte, value []byte)
func (c *Cursor) Next() (key []byte, value []byte)
func (c *Cursor) Delete() error
