package process

type Process struct{ Pid int32 }

func NewProcess(pid int32) (*Process, error)
func Processes() ([]*Process, error)
func (p *Process) Foreground() (bool, error) // proc/(pid)/stat
func (p *Process) Background() (bool, error) // proc/(pid)/stat
func (p *Process) CPUPercent() (float64, error)
func (p *Process) Children() ([]*Process, error) // pgrep -P (pid)
// ...
