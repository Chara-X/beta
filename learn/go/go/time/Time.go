package time

import "time"

type Time struct{ t time.Time }

func Date(year, month, day, hour, min, sec, nsec int, loc *time.Location) Time {
	return Time{t: time.Date(year, time.Month(month), day, hour, min, sec, nsec, loc)}
}
func Unix(sec, nsec int64) Time { return Time{t: time.Unix(sec, nsec)} }
func Parse(layout, value string) (Time, error) {
	var t, err = time.Parse(layout, value)
	t.Local()
	return Time{t: t}, err
}
func Now() Time                            { return Time{t: time.Now()} }
func (t Time) Year() int                   { return t.t.Year() } // ...
func (t Time) Unix() int64                 { return t.t.Unix() }
func (t Time) Format(layout string) string { return t.t.Format(layout) }
func (t Time) In(loc *time.Location) Time  { return Time{t: t.t.In(loc)} }
