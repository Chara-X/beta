package syscall

func Mount(source string, target string, fstype string, flags uintptr, data string) error
