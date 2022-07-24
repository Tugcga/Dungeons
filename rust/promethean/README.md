## How to build

Simply

```
wasm-pack build --target web
```

or 

```
wasm-pack build --target nodejs
```

## How to use

Import WASM module. We will use NodeJS version

```
const wasm_module = require("./promethean_node.js");
```

Create generator object

```
const generator = new wasm_module.LevelGenerator(
	16, // level width
	16, // level height
	5, // room minimal width
	7, // room maximal width
	5, // room minimal height
	7, // room maximal height
	45, // rooms count
	1, // random seed
	1, // level border
	1, // room border
	true, // generate square rooms
	true, // generate rectangular rooms
	true, // generate cross rooms
	true // generate diamond rooms
);
```

Generate a level

```
const level = generator.generate();
```

Get level tiles. It returns plain array with integers from 0 to 13

```
const tiles = level.render();
```

Get actual level height and width. These values are ```x2``` of the original size

```
const level_height = level.height();
const level_width = level.width();
```

Get some additional level statistics

```
const stat = level.statistics();
```

Extract data from statistics object

```
const stat_rooms = stat.rooms_count;
const stat_corridors = stat.corridors_count;
const stat_all_corridors = stat.all_corridors;
const stat_centers = stat.room_centers;
```


## Module API

#### LevelGenerator class

```new LevelGenerator(level_width: usize, level_height: usize, min_room_width: usize, max_room_width: usize, min_room_height: usize, max_room_height: usize, number_of_rooms: usize, random_seed: usize, border: usize, room_border: usize, room_square: bool, room_rect: bool, room_cross: bool, room_diamond: bool)```
					  
Create new ```LevelGenerator``` object. Input parameters:

* ```level_width``` the width of the level
* ```level_height``` the height of the level
* ```min_room_width``` and ```max_room_width``` the minimum and maximum width of rooms
* ```min_room_height``` and ```max_room_height``` the minimum and maximum height of rooms
* ```number_of_rooms``` target number of rooms in the level
* ```random_seed``` seed for random number generator
* ```border``` the size of the level border
* ```room_border``` the size of each room's border
* ```room_square```, ```room_rect```, ```room_cross```, ```room_diamond``` available room shapes

```LevelGenerator.set_level_size(height: usize, width: usize)```

Set height and width of the level. Input parameters:

* ```height``` the height of the level
* ```width``` the width of the level

```LevelGenerator.set_room_size(min_room_width: usize, max_room_width: usize, min_room_height: usize, max_room_height: usize)```
								
Set minimal and maximal room width and height. Input parameters:

* ```min_room_width``` and ```max_room_width``` the minimum and maximum width of rooms
* ```min_room_height``` and ```max_room_height``` the minimum and maximum height of rooms
								
```LevelGenerator.set_rooms_count(number_of_rooms: usize)```

Set the target number of rooms in the level. Input parameters:

* ```number_of_rooms``` target number of rooms

```LevelGenerator.set_seed(random_seed: usize)```

Set the random seed. Input parameters:

* ```random_seed``` seed for random number generator

```LevelGenerator.set_borders(level_border: usize, room_border: usize)```

Set level and room border size. Input parameters:

* ```level_border``` the size of the level border
* ```room_border``` the size of each room's border

```LevelGenerator.add_room_type(room_type: u8)```

Add allowed room type. ```0``` - square room, ```1``` - rectangular room, ```2``` - cross room, ```3``` - diamond room. Input parameters:

* ```room_type``` room type

```LevelGenerator.remove_room_type(room_type: u8)```

Set the room type unavailable. Input parameters:

* ```room_type``` room type

```LevelGenerator.generate() -> Level```

Generate the level. Return the object of the ```Level``` class


#### Level class

```Level.height() -> usize```

Return the height of the level

```Level.width() -> usize```

Return the width of the level

```Level.render() -> Uint8Array```

Return tiles of the level as plain array. The size of the array is ```height x width```. The first ```width``` elements are tiles of the first row, next tiles of the second row and so on. Tile values are:

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

```Level.statistics() -> LevelStatistics```

Return the ```LevelStatistics``` class object

#### LevelStatistics class

```LevelStatistics.rooms_count: usize```

The number of rooms in the level

```LevelStatistics.corridors_count: usize```

The number of generated corridors between rooms

```LevelStatistics.all_corridors: bool```

```true``` if all rooms connected by corridors, otherwise ```false```. The generator order rooms and try to find the path from each room to the next in the ordering. So, all rooms are connected if there are exactly ```rooms_count - ``` corridors

```LevelStatistics.room_centers: Int32Array```

Return the plain array with generated room centers. The first two values are coordinates of the first room center, then of the second and so on

