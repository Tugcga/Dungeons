
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0;
function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
*/
export class Level {

    static __wrap(ptr) {
        const obj = Object.create(Level.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_level_free(ptr);
    }
    /**
    * @returns {LevelStatistics}
    */
    statistics() {
        const ret = wasm.level_statistics(this.ptr);
        return LevelStatistics.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    height() {
        const ret = wasm.level_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    width() {
        const ret = wasm.level_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Array<any>}
    */
    render() {
        const ret = wasm.level_render(this.ptr);
        return takeObject(ret);
    }
}
/**
*/
export class LevelGenerator {

    static __wrap(ptr) {
        const obj = Object.create(LevelGenerator.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_levelgenerator_free(ptr);
    }
    /**
    * @param {number} level_width
    * @param {number} level_height
    * @param {number} min_room_width
    * @param {number} max_room_width
    * @param {number} min_room_height
    * @param {number} max_room_height
    * @param {number} number_of_rooms
    * @param {number} random_seed
    * @param {number} border
    * @param {number} room_border
    * @param {boolean} room_square
    * @param {boolean} room_rect
    * @param {boolean} room_cross
    * @param {boolean} room_diamond
    */
    constructor(level_width, level_height, min_room_width, max_room_width, min_room_height, max_room_height, number_of_rooms, random_seed, border, room_border, room_square, room_rect, room_cross, room_diamond) {
        const ret = wasm.levelgenerator_new(level_width, level_height, min_room_width, max_room_width, min_room_height, max_room_height, number_of_rooms, random_seed, border, room_border, room_square, room_rect, room_cross, room_diamond);
        return LevelGenerator.__wrap(ret);
    }
    /**
    * @param {number} height
    * @param {number} width
    */
    set_level_size(height, width) {
        wasm.levelgenerator_set_level_size(this.ptr, height, width);
    }
    /**
    * @param {number} min_room_width
    * @param {number} max_room_width
    * @param {number} min_room_height
    * @param {number} max_room_height
    */
    set_room_size(min_room_width, max_room_width, min_room_height, max_room_height) {
        wasm.levelgenerator_set_room_size(this.ptr, min_room_width, max_room_width, min_room_height, max_room_height);
    }
    /**
    * @param {number} number_of_rooms
    */
    set_rooms_count(number_of_rooms) {
        wasm.levelgenerator_set_rooms_count(this.ptr, number_of_rooms);
    }
    /**
    * @param {number} random_seed
    */
    set_seed(random_seed) {
        wasm.levelgenerator_set_seed(this.ptr, random_seed);
    }
    /**
    * @param {number} level_border
    * @param {number} room_border
    */
    set_borders(level_border, room_border) {
        wasm.levelgenerator_set_borders(this.ptr, level_border, room_border);
    }
    /**
    * @param {number} room_type
    */
    add_room_type(room_type) {
        wasm.levelgenerator_add_room_type(this.ptr, room_type);
    }
    /**
    * @param {number} room_type
    */
    remove_room_type(room_type) {
        wasm.levelgenerator_remove_room_type(this.ptr, room_type);
    }
    /**
    * @returns {Level}
    */
    generate() {
        const ret = wasm.levelgenerator_generate(this.ptr);
        return Level.__wrap(ret);
    }
}
/**
*/
export class LevelStatistics {

    static __wrap(ptr) {
        const obj = Object.create(LevelStatistics.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_levelstatistics_free(ptr);
    }
    /**
    */
    get rooms_count() {
        const ret = wasm.level_height(this.ptr);
        return ret >>> 0;
    }
    /**
    */
    get corridors_count() {
        const ret = wasm.level_width(this.ptr);
        return ret >>> 0;
    }
    /**
    */
    get all_corridors() {
        const ret = wasm.levelstatistics_all_corridors(this.ptr);
        return ret !== 0;
    }
    /**
    */
    get room_centers() {
        const ret = wasm.levelstatistics_room_centers(this.ptr);
        return takeObject(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_new_2ab697f1555e0dbc = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_811c8b08bf4ff9d5 = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);


    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('promethean_web_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
