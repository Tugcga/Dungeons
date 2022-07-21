async function instantiate(module, imports = {}) {
  const adaptedImports = {
    env: Object.assign(Object.create(globalThis), imports.env || {}, {
      abort(message, fileName, lineNumber, columnNumber) {
        // ~lib/builtins/abort(~lib/string/String | null?, ~lib/string/String | null?, u32?, u32?) => void
        message = __liftString(message >>> 0);
        fileName = __liftString(fileName >>> 0);
        lineNumber = lineNumber >>> 0;
        columnNumber = columnNumber >>> 0;
        (() => {
          // @external.js
          throw Error(`${message} in ${fileName}:${lineNumber}:${columnNumber}`);
        })();
      },
      seed() {
        // ~lib/builtins/seed() => f64
        return (() => {
          // @external.js
          return Date.now() * Math.random();
        })();
      },
    }),
  };
  const { exports } = await WebAssembly.instantiate(module, adaptedImports);
  const memory = exports.memory || imports.env.memory;
  const adaptedExports = Object.setPrototypeOf({
    create_generator(level_height, level_width, min_room_width, max_room_width, min_room_height, max_room_height, number_of_rooms, border, room_border, overlap_rooms, rooms_square, rooms_rectangle, rooms_cross, rooms_diamond) {
      // assembly/promethean/create_generator(i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, bool, bool) => assembly/level_generator/LevelGenerator
      overlap_rooms = overlap_rooms ? 1 : 0;
      rooms_square = rooms_square ? 1 : 0;
      rooms_rectangle = rooms_rectangle ? 1 : 0;
      rooms_cross = rooms_cross ? 1 : 0;
      rooms_diamond = rooms_diamond ? 1 : 0;
      return __liftInternref(exports.create_generator(level_height, level_width, min_room_width, max_room_width, min_room_height, max_room_height, number_of_rooms, border, room_border, overlap_rooms, rooms_square, rooms_rectangle, rooms_cross, rooms_diamond) >>> 0);
    },
    generate(generator) {
      // assembly/promethean/generate(assembly/level_generator/LevelGenerator) => assembly/level/Level
      generator = __lowerInternref(generator) || __notnull();
      return __liftInternref(exports.generate(generator) >>> 0);
    },
    level_size(level, index) {
      // assembly/promethean/level_size(assembly/level/Level, i32) => i32
      level = __lowerInternref(level) || __notnull();
      return exports.level_size(level, index);
    },
    level_tiles(level) {
      // assembly/promethean/level_tiles(assembly/level/Level) => ~lib/staticarray/StaticArray<i32>
      level = __lowerInternref(level) || __notnull();
      return __liftStaticArray(pointer => new Int32Array(memory.buffer)[pointer >>> 2], 2, exports.level_tiles(level) >>> 0);
    },
    level_statistics(level) {
      // assembly/promethean/level_statistics(assembly/level/Level) => assembly/level/LevelStatistics
      level = __lowerInternref(level) || __notnull();
      return __liftRecord19(exports.level_statistics(level) >>> 0);
    },
    statistics_valid(statistics) {
      // assembly/promethean/statistics_valid(assembly/level/LevelStatistics) => bool
      statistics = __lowerRecord19(statistics) || __notnull();
      return exports.statistics_valid(statistics) != 0;
    },
    statistics_rooms_count(statistics) {
      // assembly/promethean/statistics_rooms_count(assembly/level/LevelStatistics) => i32
      statistics = __lowerRecord19(statistics) || __notnull();
      return exports.statistics_rooms_count(statistics);
    },
    statistics_corridors_count(statistics) {
      // assembly/promethean/statistics_corridors_count(assembly/level/LevelStatistics) => i32
      statistics = __lowerRecord19(statistics) || __notnull();
      return exports.statistics_corridors_count(statistics);
    },
    statistics_complete_corridors(statistics) {
      // assembly/promethean/statistics_complete_corridors(assembly/level/LevelStatistics) => bool
      statistics = __lowerRecord19(statistics) || __notnull();
      return exports.statistics_complete_corridors(statistics) != 0;
    },
    statistics_room_centers(statistics) {
      // assembly/promethean/statistics_room_centers(assembly/level/LevelStatistics) => ~lib/staticarray/StaticArray<i32>
      statistics = __lowerRecord19(statistics) || __notnull();
      return __liftStaticArray(pointer => new Int32Array(memory.buffer)[pointer >>> 2], 2, exports.statistics_room_centers(statistics) >>> 0);
    },
  }, exports);
  function __liftRecord19(pointer) {
    // assembly/level/LevelStatistics
    // Hint: Opt-out from lifting as a record by providing an empty constructor
    if (!pointer) return null;
    return {
      init: new Uint8Array(memory.buffer)[pointer + 0 >>> 0] != 0,
      rooms_count: new Int32Array(memory.buffer)[pointer + 4 >>> 2],
      corridors_count: new Int32Array(memory.buffer)[pointer + 8 >>> 2],
      all_corridors: new Uint8Array(memory.buffer)[pointer + 12 >>> 0] != 0,
      room_centers: __liftStaticArray(pointer => __liftInternref(new Uint32Array(memory.buffer)[pointer >>> 2]), 2, new Uint32Array(memory.buffer)[pointer + 16 >>> 2]),
    };
  }
  function __lowerRecord19(value) {
    // assembly/level/LevelStatistics
    // Hint: Opt-out from lowering as a record by providing an empty constructor
    if (value == null) return 0;
    const pointer = exports.__pin(exports.__new(20, 19));
    new Uint8Array(memory.buffer)[pointer + 0 >>> 0] = value.init ? 1 : 0;
    new Int32Array(memory.buffer)[pointer + 4 >>> 2] = value.rooms_count;
    new Int32Array(memory.buffer)[pointer + 8 >>> 2] = value.corridors_count;
    new Uint8Array(memory.buffer)[pointer + 12 >>> 0] = value.all_corridors ? 1 : 0;
    new Uint32Array(memory.buffer)[pointer + 16 >>> 2] = __lowerStaticArray((pointer, value) => { new Uint32Array(memory.buffer)[pointer >>> 2] = __lowerInternref(value) || __notnull(); }, 20, 2, value.room_centers) || __notnull();
    exports.__unpin(pointer);
    return pointer;
  }
  function __liftString(pointer) {
    if (!pointer) return null;
    const
      end = pointer + new Uint32Array(memory.buffer)[pointer - 4 >>> 2] >>> 1,
      memoryU16 = new Uint16Array(memory.buffer);
    let
      start = pointer >>> 1,
      string = "";
    while (end - start > 1024) string += String.fromCharCode(...memoryU16.subarray(start, start += 1024));
    return string + String.fromCharCode(...memoryU16.subarray(start, end));
  }
  function __liftStaticArray(liftElement, align, pointer) {
    if (!pointer) return null;
    const
      length = new Uint32Array(memory.buffer)[pointer - 4 >>> 2] >>> align,
      values = new Array(length);
    for (let i = 0; i < length; ++i) values[i] = liftElement(pointer + (i << align >>> 0));
    return values;
  }
  function __lowerStaticArray(lowerElement, id, align, values) {
    if (values == null) return 0;
    const
      length = values.length,
      buffer = exports.__pin(exports.__new(length << align, id)) >>> 0;
    for (let i = 0; i < length; i++) lowerElement(buffer + (i << align >>> 0), values[i]);
    exports.__unpin(buffer);
    return buffer;
  }
  const registry = new FinalizationRegistry(__release);
  class Internref extends Number {}
  function __liftInternref(pointer) {
    if (!pointer) return null;
    const sentinel = new Internref(__retain(pointer));
    registry.register(sentinel, pointer);
    return sentinel;
  }
  function __lowerInternref(value) {
    if (value == null) return 0;
    if (value instanceof Internref) return value.valueOf();
    throw TypeError("internref expected");
  }
  const refcounts = new Map();
  function __retain(pointer) {
    if (pointer) {
      const refcount = refcounts.get(pointer);
      if (refcount) refcounts.set(pointer, refcount + 1);
      else refcounts.set(exports.__pin(pointer), 1);
    }
    return pointer;
  }
  function __release(pointer) {
    if (pointer) {
      const refcount = refcounts.get(pointer);
      if (refcount === 1) exports.__unpin(pointer), refcounts.delete(pointer);
      else if (refcount) refcounts.set(pointer, refcount - 1);
      else throw Error(`invalid refcount '${refcount}' for reference '${pointer}'`);
    }
  }
  function __notnull() {
    throw TypeError("value must not be null");
  }
  return adaptedExports;
}
export const {
  create_generator,
  generate,
  level_size,
  level_tiles,
  level_statistics,
  statistics_valid,
  statistics_rooms_count,
  statistics_corridors_count,
  statistics_complete_corridors,
  statistics_room_centers
} = await (async url => instantiate(
  await (async () => {
    try { return await globalThis.WebAssembly.compileStreaming(globalThis.fetch(url)); }
    catch { return globalThis.WebAssembly.compile(await (await import("node:fs/promises")).readFile(url)); }
  })(), {
  }
))(new URL("promethean.wasm", import.meta.url));
