package metadata

import "context"

func NewOutgoingContext(ctx context.Context, md map[string][]string) context.Context
