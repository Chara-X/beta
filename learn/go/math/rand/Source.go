package rand

type Source interface {
	Seed(seed int64)
	Int63() int64
}

func NewSource(seed int64) Source

// type Rand struct {
// 	source Source
// }

// func New(source Source) *Rand
// func (rand *Rand) Seed(seed int64) {
// 	rand.source.Seed(seed)
// }
// func (rand *Rand) Int63() int64 {
// 	return rand.source.Int63()
// }
// func (rand *Rand) Int63n(n int64) int64 {
// 	return rand.Int63() % n
// }
// func (rand *Rand) Float64() float64 {
// 	return float64(rand.Int63()) / (1 << 63)
// }
// func (rand *Rand) Shuffle(n int, swap func(i, j int)) {
// 	for i := n - 1; i > 0; i-- {
// 		j := rand.Int63n(int64(i + 1))
// 		swap(i, int(j))
// 	}
// }
// func (rand *Rand) Perm(n int) []int {
// 	var perm = make([]int, n)
// 	for i := 0; i < n; i++ {
// 		perm[i] = i
// 	}
// 	rand.Shuffle(n, func(i, j int) {
// 		perm[i], perm[j] = perm[j], perm[i]
// 	})
// 	return perm
// }
