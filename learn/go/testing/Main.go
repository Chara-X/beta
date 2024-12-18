package testing

func Main(tests []struct {
	Name string
	F    func(*T)
}, examples []struct {
	Name      string
	F         func()
	Output    string
	Unordered bool
})

/*
Main(nil, tests, nil, nil, examples)
*/
