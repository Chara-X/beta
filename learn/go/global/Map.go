package global

type Map[K comparable, V any] map[K]V

func (m *Map[K, V]) Delete(key K) { delete(*m, key) }
func (m *Map[K, V]) Clear()       { clear(*m) }
