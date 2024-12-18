package procfs

type FS struct{}

func NewFS(mountPoint string) (FS, error)
func (fs FS) Proc(pid int) (Proc, error)
func (fs FS) AllProcs() ([]Proc, error)

// ...
