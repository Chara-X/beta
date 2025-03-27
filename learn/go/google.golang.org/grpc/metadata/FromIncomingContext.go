package metadata

import "context"

func FromIncomingContext(ctx context.Context) (map[string][]string, bool)
