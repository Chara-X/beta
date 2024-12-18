package proto

import "google.golang.org/protobuf/proto"

func Unmarshal(buf []byte, msg proto.Message) error
