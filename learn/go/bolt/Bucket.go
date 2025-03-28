package bolt

type Bucket struct {
	FillPercent float64
	Sequence    uint64
}

func (b *Bucket) CreateBucket(key []byte) (*Bucket, error)
func (b *Bucket) DeleteBucket(key []byte) error
func (b *Bucket) Bucket(key []byte) *Bucket
func (b *Bucket) Put(key []byte, value []byte) error
func (b *Bucket) Delete(key []byte) error
func (b *Bucket) Get(key []byte) []byte
func (b *Bucket) Cursor() *Cursor

// Sequence: Sequence, SetSequence, NextSequence
