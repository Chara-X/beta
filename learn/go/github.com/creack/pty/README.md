# pty

```go
func ExamplePty() {
	var c = exec.Command("/bin/bash")
	var p, t, _ = pty.Open()
	defer p.Close()
	defer t.Close()
	c.Stdin = t
	c.Stdout = t
	c.Stderr = t
	c.SysProcAttr = &syscall.SysProcAttr{Setctty: true, Setsid: true, Ctty: 0}
	c.Start()
	defer c.Process.Kill()
	go io.Copy(os.Stdout, p)
	io.Copy(p, os.Stdin)
}
```
