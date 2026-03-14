# ruler-js

WebAssembly bindings for the [ruler](https://github.com/scristobal/geo-ruler) geodesic calculations library.

Provides a `Coords` class for fast city-scale distance, bearing, and destination calculations from JavaScript using the Cheap Ruler approximation algorithm with the WGS84 ellipsoid.

## Build

```bash
cargo install wasm-pack
wasm-pack build --target web --out-dir pkg
```

## Usage

```javascript
import init, { Coords } from './pkg/ruler_js.js';

async function main() {
    await init();

    const empireState = new Coords(-73.9857, 40.7484);
    const flatiron = new Coords(-73.9897, 40.7411);

    const distance = empireState.distance(flatiron);
    console.log(`Distance: ${distance.toFixed(1)} meters`);

    const bearing = empireState.bearing(flatiron);
    console.log(`Bearing: ${bearing.toFixed(1)} degrees`);

    const destination = empireState.destination(45.0, 1000.0);
    console.log(`Destination: ${destination.x}, ${destination.y}`);
}

main();
```

## API

### `Coords`

- `new Coords(x, y)` - Create a coordinate point (longitude, latitude)
- `coords.distance(destination)` - Distance in meters to another coordinate
- `coords.bearing(destination)` - Bearing in degrees (0-360) to another coordinate
- `coords.destination(bearing, distance)` - New coordinate at given bearing and distance
- `coords.x` / `coords.y` - Longitude / Latitude

## License

MIT
