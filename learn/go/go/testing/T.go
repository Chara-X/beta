package testing

import (
	"context"
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

// [testing.T]
type T struct{ t *testing.T }

func (c *T) Name() string     { return c.t.Name() }
func (c *T) TempDir() string  { return c.t.TempDir() }
func (c *T) Log(args ...any)  { c.t.Log(args...) }
func (c *T) Fatal()           { c.t.Fatal() }
func (c *T) Cleanup(f func()) { c.t.Cleanup(f) }

// [testing.T.Chdir]
func (t *T) Chdir(dir string) {
	oldwd, _ := os.Open(".")
	os.Chdir(dir)
	if !filepath.IsAbs(dir) {
		dir, _ = os.Getwd()
	}
	t.Setenv("PWD", dir)
	t.Cleanup(func() {
		oldwd.Chdir()
		oldwd.Close()
	})
}

// [testing.T.Setenv]
func (t *T) Setenv(key, value string) {
	prevValue, ok := os.LookupEnv(key)
	os.Setenv(key, value)
	if ok {
		t.Cleanup(func() {
			os.Setenv(key, prevValue)
		})
	} else {
		t.Cleanup(func() {
			os.Unsetenv(key)
		})
	}
}

// [testing.T.Run]
func (t *T) Run(name string, f func(t *T)) bool {
	testName, ok, _ := t.tstate.match.fullName(&t.common, name)
	if !ok || shouldFailFast() {
		return true
	}
	// Record the stack trace at the point of this call so that if the subtest
	// function - which runs in a separate stack - is marked as a helper, we can
	// continue walking the stack into the parent test.
	var pc [maxStackLen]uintptr
	n := runtime.Callers(2, pc[:])
	// There's no reason to inherit this context from parent. The user's code can't observe
	// the difference between the background context and the one from the parent test.
	ctx, cancelCtx := context.WithCancel(context.Background())
	t = &T{
		common: common{
			barrier:   make(chan bool),
			signal:    make(chan bool, 1),
			name:      testName,
			parent:    &t.common,
			level:     t.level + 1,
			creator:   pc[:n],
			chatty:    t.chatty,
			ctx:       ctx,
			cancelCtx: cancelCtx,
		},
		tstate: t.tstate,
	}
	t.w = indenter{&t.common}
	if t.chatty != nil {
		t.chatty.Updatef(t.name, "=== RUN   %s\n", t.name)
	}
	running.Store(t.name, highPrecisionTimeNow())
	// Instead of reducing the running count of this test before calling the
	// tRunner and increasing it afterwards, we rely on tRunner keeping the
	// count correct. This ensures that a sequence of sequential tests runs
	// without being preempted, even when their parent is a parallel test. This
	// may especially reduce surprises if *parallel == 1.
	go tRunner(t, f)
	// The parent goroutine will block until the subtest either finishes or calls
	// Parallel, but in general we don't know whether the parent goroutine is the
	// top-level test function or some other goroutine it has spawned.
	// To avoid confusing false-negatives, we leave the parent in the running map
	// even though in the typical case it is blocked.
	if !<-t.signal {
		// At this point, it is likely that FailNow was called on one of the
		// parent tests by one of the subtests. Continue aborting up the chain.
		runtime.Goexit()
	}
	if t.chatty != nil && t.chatty.json {
		t.chatty.Updatef(t.parent.name, "=== NAME  %s\n", t.parent.name)
	}
	return !t.failed
}
