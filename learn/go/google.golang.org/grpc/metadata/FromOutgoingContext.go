package metadata

import "context"

func FromOutgoingContext(ctx context.Context) (map[string][]string, bool)
