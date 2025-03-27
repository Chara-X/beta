package sync

type WaitGroup struct{}

func (group *WaitGroup) Add(delta int)
func (group *WaitGroup) Wait()
