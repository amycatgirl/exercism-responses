// Package weather provides tooling to show the user a certain city's forecast.
package weather

// CurrentCondition represents the current condition of CurrentLocation.
var CurrentCondition string

// CurrentLocation represents the currently selected location.
var CurrentLocation string

// Forecast returns a string with the current location and condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
