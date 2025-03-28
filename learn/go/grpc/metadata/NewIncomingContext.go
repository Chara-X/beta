package metadata

import "context"

func NewIncomingContext(ctx context.Context, md map[string][]string) context.Context
