/**
 * assembly/promethean/create_generator
 * @param level_height `i32`
 * @param level_width `i32`
 * @param min_room_width `i32`
 * @param max_room_width `i32`
 * @param min_room_height `i32`
 * @param max_room_height `i32`
 * @param number_of_rooms `i32`
 * @param border `i32`
 * @param room_border `i32`
 * @param overlap_rooms `bool`
 * @param rooms_square `bool`
 * @param rooms_rectangle `bool`
 * @param rooms_cross `bool`
 * @param rooms_diamond `bool`
 * @returns `assembly/level_generator/LevelGenerator`
 */
export declare function create_generator(level_height: number, level_width: number, min_room_width: number, max_room_width: number, min_room_height: number, max_room_height: number, number_of_rooms: number, border: number, room_border: number, overlap_rooms: boolean, rooms_square: boolean, rooms_rectangle: boolean, rooms_cross: boolean, rooms_diamond: boolean): __Internref10;
/**
 * assembly/promethean/generate
 * @param generator `assembly/level_generator/LevelGenerator`
 * @returns `assembly/level/Level`
 */
export declare function generate(generator: __Internref10): __Internref18;
/**
 * assembly/promethean/level_size
 * @param level `assembly/level/Level`
 * @param index `i32`
 * @returns `i32`
 */
export declare function level_size(level: __Internref18, index: number): number;
/**
 * assembly/promethean/level_tiles
 * @param level `assembly/level/Level`
 * @returns `~lib/staticarray/StaticArray<i32>`
 */
export declare function level_tiles(level: __Internref18): Array<number>;
/**
 * assembly/promethean/level_statistics
 * @param level `assembly/level/Level`
 * @returns `assembly/level/LevelStatistics`
 */
export declare function level_statistics(level: __Internref18): __Record19<never>;
/**
 * assembly/promethean/statistics_valid
 * @param statistics `assembly/level/LevelStatistics`
 * @returns `bool`
 */
export declare function statistics_valid(statistics: __Record19<undefined>): boolean;
/**
 * assembly/promethean/statistics_rooms_count
 * @param statistics `assembly/level/LevelStatistics`
 * @returns `i32`
 */
export declare function statistics_rooms_count(statistics: __Record19<undefined>): number;
/**
 * assembly/promethean/statistics_corridors_count
 * @param statistics `assembly/level/LevelStatistics`
 * @returns `i32`
 */
export declare function statistics_corridors_count(statistics: __Record19<undefined>): number;
/**
 * assembly/promethean/statistics_complete_corridors
 * @param statistics `assembly/level/LevelStatistics`
 * @returns `bool`
 */
export declare function statistics_complete_corridors(statistics: __Record19<undefined>): boolean;
/**
 * assembly/promethean/statistics_room_centers
 * @param statistics `assembly/level/LevelStatistics`
 * @returns `~lib/staticarray/StaticArray<i32>`
 */
export declare function statistics_room_centers(statistics: __Record19<undefined>): Array<number>;
/** assembly/level_generator/LevelGenerator */
declare class __Internref10 extends Number {
  private __nominal10: symbol;
}
/** assembly/level/Level */
declare class __Internref18 extends Number {
  private __nominal18: symbol;
}
/** assembly/point/Point */
declare class __Internref3 extends Number {
  private __nominal3: symbol;
}
/** assembly/level/LevelStatistics */
declare interface __Record19<TOmittable> {
  /** @type `bool` */
  init: boolean | TOmittable;
  /** @type `i32` */
  rooms_count: number | TOmittable;
  /** @type `i32` */
  corridors_count: number | TOmittable;
  /** @type `bool` */
  all_corridors: boolean | TOmittable;
  /** @type `~lib/staticarray/StaticArray<assembly/point/Point>` */
  room_centers: Array<__Internref3>;
}
