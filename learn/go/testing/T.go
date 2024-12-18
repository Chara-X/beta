package testing

type T struct{}

func (t *T) Name() string
func (t *T) Run(name string, f func(t *T)) bool
func (t *T) Parallel()
func (t *T) Log(args ...any)
func (t *T) Skip()
func (t *T) Fatal()
