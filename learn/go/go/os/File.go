package os

import "os"

type File struct{}

func OpenFile(name string, flag int, permissions os.FileMode) (*File, error)
func (file *File) Stat() (os.FileInfo, error)
func (file *File) Readdir(n int) ([]os.FileInfo, error)
func (file *File) Read(buffer []byte) (n int, err error)
func (file *File) Write(buffer []byte) (n int, err error)
func (file *File) Seek(offset int64, whence int) (position int64, err error)
func (file *File) Close() error

/*
OpenFile: OpenFile + Mkdir
*/
