## How to use

Import classes from the library

```
from promethean import LevelGenerator, Level, LevelStatistics, Options, Tile, RoomType
```

Create options

```
options: Options = Options()
```

Setup parameters

```
options.level_width = 32
options.level_height = 32
options.overlap_rooms = False
options.room_types = [RoomType.Rectangle]
```

Create generator

```
generator: LevelGenerator = LevelGenerator(options)
```

Generate the level

```
level: Level = generator.generate()
```

Next you can obtains some statistics from the level

```
level_stat: LevelStatistics = level.get_statistics()
```

Also you can get tiles of the level

```
tiles: List[List[Tile]] = level.render()
```

Tiles is 2d-array, which contains type of the level tiles. ```0``` is a walkable tile, all other tiles are not-walkable. ```1``` is empty tile, ```2 - 13``` are walls of different type.


## Example application

Repository contains example application with GUI (based on PySide6). Run it

```
python pyside_example.py
```

It draws generated map into canvas and allows to tweak generation parameters and 

![Application example](../images/app_01.png?raw=true)
