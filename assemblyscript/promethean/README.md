## How to build

```
asc assembly/promethean.ts --outFile build/promethean.wasm --bindings esm --exportRuntime
```

## How to use

The module compiled with AssemblyScript compiler 0.20.1. It does not supports classes export. All communications between JS and the module use exported functions.

Import the module

```
import * as exports from "./promethean.js";
```

Create the generator and store in the variable the pointer to the generator object inside WASM memory

```
let gen_ptr = exports.create_generator(64, 64, 3, 7, 3, 7, 20, 1, 1, false, true, true, true, true);
```

Generate the level. Also store the pointer to the level object

```
let level_ptr = exports.generate(gen_ptr);
```

Get the level height and width. In fact these values are ```x2``` with respect to the size, defined for the generator

```
const height = exports.level_size(level_ptr, 0);
const width = exports.level_size(level_ptr, 1);
```

Get the plain array of level tiles. The size of this array ```height x width```. Values are integers ```0, 1, ..., 13```. This array describe the geometry of the generated level

```
let level_tiles = exports.level_tiles(level_ptr);
```

It's possible to get some additional data of the generated level. Get the pointer to the statistics object

```
const stat_ptr = exports.level_statistics(level_ptr);
```

The number rooms in the level. This value can be smaller than value define for the generator

```
const stat_rooms = exports.statistics_rooms_count(stat_ptr);
```

The number of corridors between rooms

```
const stat_corridors = exports.statistics_corridors_count(stat_ptr);
```

Plain array with coordinates of room centers. The size of the array is ```x2``` with respect to the number of rooms

```
const stat_centers = exports.statistics_room_centers(stat_ptr);
```


## Module API

```
function create_generator(level_height: i32,
						  level_width: i32,
						  min_room_width: i32,
						  max_room_width: i32,
						  min_room_height: i32,
						  max_room_height: i32,
						  number_of_rooms: i32,
						  border: i32,
						  room_border: i32,
						  overlap_rooms: bool,
						  rooms_square: bool,
						  rooms_rectangle: bool,
						  rooms_cross: bool,
						  rooms_diamond: bool): LevelGenerator;
```

Create the generator object and return the pointer to it. Input parameters:

* ```level_height: i32``` the height of the level. Actual height will be ```x2```
* ```level_width: i32``` the width of the level. Actual width will be ```x2```
* ```min_room_width: i32``` and ```max_room_width: i32``` minimum and maximum width of rooms
* ```min_room_height: i32``` and ```max_room_height: i32``` minimum and maximum height of rooms
* ```number_of_rooms: i32``` the target count of rooms in the level. Actual value can be smaller
* ```border: i32``` the border size of the level. Tiles in the border are always empty
* ```room_border: i32``` the border size of each room
* ```overlap_rooms: bool``` if ```false``` then rooms will be placed without intersections, if ```true``` then the generator will generate exactly ```number_of_rooms``` rooms
* ```rooms_square: bool``` if ```true``` then the level will be contains square rooms
* ```rooms_rectangle: bool``` if ```true``` then the level will be contains rectangular rooms
* ```rooms_cross: bool``` if ```true``` then the level will be contains rooms with cross shape
* ```rooms_diamond: bool``` if ```true``` then the level will be contains rooms with diamond shape

```
function generate(generator: LevelGenerator): Level;
```

Generate the level and return the pointer to it. Input parameters:

* ```generator: LevelGenerator``` the pointer to the generator object

```
function level_size(level: Level, index: i32): i32;
```

Return the size of the generated level. Input parameters:

* ```level: Level``` the pointer to the level object
* ```index: i32``` if ```0```, then return the height of the level, if ```1```, then return the width of the level

```
function level_tiles(level: Level): StaticArray<Tile>;
```

Return the plain array of level tiles. Input parameters:

* ```level: Level``` the pointer to the level object

Tile values are:

* ```0``` floor
* ```1``` empty
* ```2``` top left inside corner
* ```3``` top right inside corner
* ```4``` bottom left inside corner
* ```5``` bottom right inside corner
* ```6``` top wall
* ```7``` right wall
* ```8``` bottom wall
* ```9``` left wall
* ```10``` top left outside corner
* ```11``` top right outside corner
* ```12``` bottom left outside corner
* ```13``` bottom right outside corner

```
function level_statistics(level: Level): LevelStatistic;
```

Return pointer to the statistics object. Input parameters:

* ```level: Level``` pointer to the level object

```
function statistics_valid(statistics: LevelStatistics): bool;
```

Return ```true``` if the level is generated, and ```false``` otherwise. Input parameters:

* ```statistics: LevelStatistics``` pointer to the statistic object

```
function statistics_rooms_count(statistics: LevelStatistics): i32;
```

Return the number of rooms in the level. Input parameters:

* ```statistics: LevelStatistics``` pointer to the statistic object

```
function statistics_corridors_count(statistics: LevelStatistics): i32;
```

Return the number of corridors in the level. Input parameters:

* ```statistics: LevelStatistics``` pointer to the statistic object

```
function statistics_complete_corridors(statistics: LevelStatistics): bool;
```

Return ```true``` if all rooms connected by corridors and ```false``` if the generator fails to build the corridor between any two rooms. Input parameters:

* ```statistics: LevelStatistics``` pointer to the statistic object

```
function statistics_room_centers(statistics: LevelStatistics): StaticArray<i32>;
```

Return plain array with coordinates of room centers. The size of the array is ```x2``` with respect to the number of rooms. The first two values are coordinates of the first room center (in the vertical direction, and then in the horizontal direction), next two values of the second room and so on. Input parameters:

* ```statistics: LevelStatistics``` pointer to the statistic object


## Example application

Example application is [here](https://tugcga.github.io/web_apps/as_promethean/example_app.html). It use compiled WASM for map generation and draw it into 2d-canvas.

![Application example](../../images/app_02.png?raw=true)