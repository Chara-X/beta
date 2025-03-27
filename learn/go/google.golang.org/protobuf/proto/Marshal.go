package proto

import "google.golang.org/protobuf/proto"

func Marshal(msg proto.Message) ([]byte, error)
