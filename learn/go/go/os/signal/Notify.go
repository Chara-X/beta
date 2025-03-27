package signal

import "os"

func Notify(channel chan<- os.Signal, signals ...os.Signal)
