
export class PseudoRandom{
    next(in_min: i32, in_max: i32): i32{
        const v: f64 = Math.random();
        return <i32>(<f64>(in_max - in_min) * v + <f64>in_min + 0.5);
    }

    next_odd(in_min: i32, in_max: i32): i32{
        const next_value: i32 = this.next(in_min, in_max);

        if(next_value % 2 != 0){
            return next_value;
        }
        else{
            if(next_value < in_max){
                return next_value + 1;
            }
            else{
                return next_value - 1;
            }
        }
    }
}
