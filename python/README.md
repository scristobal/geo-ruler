# geo-ruler-python

Python bindings for the [geo-ruler](https://github.com/scristobal/geo-ruler) geodesic calculations library.

Provides a `Coords` class for fast city-scale distance, bearing, and destination calculations from Python using the Cheap Ruler approximation algorithm with the WGS84 ellipsoid.

## Build

```bash
pip install maturin
maturin develop
```

## Usage

```python
from geo_ruler import Coords

empire_state = Coords(-73.9857, 40.7484)
flatiron = Coords(-73.9897, 40.7411)

distance = empire_state.distance(flatiron)
print(f"Distance: {distance:.1f} meters")

bearing = empire_state.bearing(flatiron)
print(f"Bearing: {bearing:.1f} degrees")

destination = empire_state.destination(45.0, 1000.0)
print(f"Destination: {destination.x}, {destination.y}")
```

## API

### `Coords`

- `Coords(x, y)` - Create a coordinate point (longitude, latitude)
- `coords.distance(destination)` - Distance in meters to another coordinate
- `coords.bearing(destination)` - Bearing in degrees (0-360) to another coordinate
- `coords.destination(bearing, distance)` - New coordinate at given bearing and distance
- `coords.x` / `coords.y` - Longitude / Latitude (read/write)

## Typing

The package includes type stubs (`py.typed` + `__init__.pyi`) for full IDE support.

## License

MIT
