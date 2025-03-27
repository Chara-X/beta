package status

import "google.golang.org/grpc/codes"

type Status struct{}

func New(code codes.Code, msg string) *Status
func FromError(err error) (*Status, bool)
func (s *Status) Code() codes.Code
func (s *Status) Message() string
func (s *Status) Err() error

// ...
