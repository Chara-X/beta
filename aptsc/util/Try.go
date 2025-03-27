package util

func Try(f func()) (err error) {
	defer func() {
		err, _ = recover().(error)
	}()
	f()
	return
}
