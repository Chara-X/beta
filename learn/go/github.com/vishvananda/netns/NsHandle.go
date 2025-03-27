package netns

type NsHandle int

func Get() (NsHandle, error)
func GetFromName(name string) (NsHandle, error)
func GetFromPid(pid int) (NsHandle, error)
func GetFromPath(path string) (NsHandle, error)
func (ns *NsHandle) Close() error

// ...
