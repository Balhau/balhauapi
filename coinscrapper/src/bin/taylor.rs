use std::f64;
extern crate test;
use test::Bencher;

const C2: f64 = 0.5;
const C4: f64 = 1.0 / 24.0;
const MAX_ITER : u64 = 1000000000;

fn t4(val: f64) -> f64 {
    let x2 = val * val;
    let x4 = x2 * x2;
    1.0 - C2 * x2 + C4 * x4
}

#[bench]
fn bench_cos_MAX_ITER_floats(b: &mut Bencher) {
    b.iter(|| {
        // use `test::black_box` to prevent compiler optimizations from disregarding
        // unused values
        test::black_box((0 .. MAX_ITER).map(|v| ((MAX_ITER as f64)/(MAX_ITER+v) as f64).cos()));
    });
}

fn main() {
    let x: f64 = 0.0;

    let f_0 = x.cos();                  //cos(0)
    let d1_0: f64 = -x.sin();                 //d(cos)  = -sin
    let d2_0: f64 = -x.cos();                 //d²(cos) = d(-sin) = -cos
    let d3_0: f64 = x.sin();                  //d³(cos) = d(-cos) = sin
    let d4_0: f64 = x.cos();

    let step = 0.05;
    for i in 0..100 {                    //iterate over -1 to 1 neighborhour of 0
        let x = -1.0 + step * (i as f64);
        let v = x.cos();
        let v_approx = t4(x);
        println!("cos({}) = {} ~ {} = {}", x, v, v_approx, v - v_approx);
    }

    let mut inc: u64 = 0;
    let x : f64 = 0.5;
    let mut res : f64 = 0.0;
    loop {
        if inc > MAX_ITER { break };
        res = x.cos();
        inc = inc + 1;
    }

    println!("{}",res);

    let mut inc: u64 = 0;
    loop{
        if inc > MAX_ITER { break };
        res = t4(x);
        inc = inc +1;
    }

    println!("{}",res);


    //Taylor approx:
    // c_0+c_1x+c_2x²+c_3x³+c_4x⁴
    //where:
    //c_0 = f(0) = x.cos()  = 1
    //c_1 = c1 = -x.sin()    = 0
    //2*c_2 = -x.cos()  => c_2 = -1/2
    //6*c_3 = x.sin() => c_3 = 0
    //24*c_4 = x.cos() => c_4 = 1/24
}