// Package weather helps you forecast the weather.
package weather

// CurrentCondition is the last user condition string.
var CurrentCondition string

// CurrentLocation is the last user location string.
var CurrentLocation string

// Forecast sets the module-level CurrentCondition and CurrentLocation vars from
// its arguments, and returns a formated string.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
