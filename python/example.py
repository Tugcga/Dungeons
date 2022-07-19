from promethean import LevelGenerator, Level, Options, Tile, RoomType

if __name__ == "__main__":
    # setup generator options
    options: Options = Options()
    options.level_width = 32  # map size
    options.level_height = 32
    options.overlap_rooms = False  # make rooms non-intersectable
    options.room_types = [RoomType.Rectangle]  # select room types
    # setup other parameters...

    # create generator
    generator = LevelGenerator(options)

    # generate the level
    level = generator.generate()

    # extract level statistics data
    level_stat = level.get_statistics()

    # get level tiles
    # this is a 2d-array, each value is a tile type
    # 0 - walkable tile, all other tiles are not walkable
    # 1 - empty 
    # 2 - 13 - different types of walls
    tiles = level.render()
