package fsnotify

import "github.com/fsnotify/fsnotify"

type Watcher struct{ Events chan fsnotify.Event }

func NewWatcher() (*Watcher, error)
func (watcher *Watcher) Add(name string) error
func (watcher *Watcher) Remove(name string) error
func (watcher *Watcher) Close() error
