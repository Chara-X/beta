package grpc

type CallOption interface{}

func CallContentSubtype(contentSubtype string) CallOption
