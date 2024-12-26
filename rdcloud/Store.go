package rdcloud

import (
	"encoding/csv"
	"os"

	_ "modernc.org/sqlite"
)

type Store map[string]string

func New(name string) *Store {
	var f, err = os.Open(name)
	if err != nil {
		panic(err)
	}
	var reader = csv.NewReader(f)
	columns, err := reader.Read()
	if err != nil {
		panic(err)
	}
	for row, err := reader.Read(); err == nil; row, err = reader.Read() {

	}
}
func (r *Store) QueryAllTroubles() []Troubleshooting {
	var rows, err = r.db.Query("SELECT * FROM troubleshooting")
	if err != nil {
		panic(err)
	}
	defer rows.Close()
	columnNames, err := rows.Columns()
	if err != nil {
		panic(err)
	}
	var res = []Troubleshooting{}
	for rows.Next() {
		var row = make([]interface{}, len(columnNames))
		for i := range row {
			var value = new(string)
			row[i] = value
		}
		if err := rows.Scan(row...); err != nil {
			panic(err)
		}
		var trouble = Troubleshooting{}
		for i, columnName := range columnNames {
			trouble[columnName] = *row[i].(*string)
		}
		res = append(res, trouble)
	}
	return res
}
func (r *Store) Close() {

}
