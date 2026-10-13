use std::collections::BinaryHeap;

use crate::{integral::adaptive::Interval, measure::Measure};

fn gauss_kronrod_7_15_integral<F>(f: F, a: f64, b: f64) -> Measure
where
    F: Fn(f64) -> f64,
{
    let mut gauss_nodes: [f64; 7] = [
        -0.949107912342759,
        -0.741531185599394,
        -0.405845151377397,
        0.949107912342759,
        0.741531185599394,
        0.405845151377397,
        0.0,
    ];

    for node in &mut gauss_nodes {
        *node = 0.5 * (b - a) * *node + 0.5 * (b + a);
    }

    let mut gauss_weights: [f64; 7] = [
        0.129484966168870,
        0.279705391489277,
        0.381830050505119,
        0.129484966168870,
        0.279705391489277,
        0.381830050505119,
        0.417959183673469,
    ];

    for weight in &mut gauss_weights {
        *weight *= 0.5 * (b - a);
    }

    let mut kronrod_nodes: [f64; 15] = [
        -0.991455371120813,
        -0.949107912342759,
        -0.864864423359769,
        -0.741531185599394,
        -0.586087235467691,
        -0.405845151377397,
        -0.207784955007898,
        0.991455371120813,
        0.949107912342759,
        0.864864423359769,
        0.741531185599394,
        0.586087235467691,
        0.405845151377397,
        0.207784955007898,
        0.0,
    ];

    for node in &mut kronrod_nodes {
        *node = 0.5 * (b - a) * *node + 0.5 * (b + a);
    }

    let mut kronrod_weights: [f64; 15] = [
        0.022935322010529,
        0.063092092629979,
        0.104790010322250,
        0.140653259715525,
        0.169004726639267,
        0.190350578064785,
        0.204432940075298,
        0.022935322010529,
        0.063092092629979,
        0.104790010322250,
        0.140653259715525,
        0.169004726639267,
        0.190350578064785,
        0.204432940075298,
        0.209482141084728,
    ];

    for weight in &mut kronrod_weights {
        *weight *= 0.5 * (b - a);
    }

    let mut gauss_integral: f64 = 0.0;
    for i in 0..7 {
        gauss_integral += f(gauss_nodes[i]) * gauss_weights[i];
    }

    let mut gauss_kronrod_integral: f64 = 0.0;
    for i in 0..15 {
        gauss_kronrod_integral += f(kronrod_nodes[i]) * kronrod_weights[i];
    }

    let error: f64 = (gauss_kronrod_integral - gauss_integral).abs();

    Measure {
        value: gauss_kronrod_integral,
        error,
    }
}

pub fn gauss_kronrod<F>(f: F, a: f64, b: f64, abs_error: f64, rel_error: f64) -> Measure
where
    F: Fn(f64) -> f64,
{
    let mut result: f64 = 0.0;
    let mut error: f64 = 0.0;

    let mut intervals: BinaryHeap<Interval> = BinaryHeap::new();

    let mut temp: Measure = gauss_kronrod_7_15_integral(&f, a, (b + a) / 2.0);
    result += temp.value;
    error += temp.error;
    intervals.push(Interval {
        a,
        b: (b + a) / 2.0,
        value: temp.value,
        error: temp.error,
    });

    temp = gauss_kronrod_7_15_integral(&f, (b + a) / 2.0, b);
    result += temp.value;
    error += temp.error;
    intervals.push(Interval {
        a: (b + a) / 2.0,
        b,
        value: temp.value,
        error: temp.error,
    });

    while error >= abs_error && error >= rel_error * result.abs() {
        let worst_interval: Interval = intervals.pop().unwrap();

        let lower_bound: f64 = worst_interval.a;
        let upper_bound: f64 = worst_interval.b;
        let middle: f64 = (upper_bound + lower_bound) / 2.0;

        result -= worst_interval.value;
        error -= worst_interval.error;

        temp = gauss_kronrod_7_15_integral(&f, lower_bound, middle);
        intervals.push(Interval {
            a: lower_bound,
            b: middle,
            value: temp.value,
            error: temp.error,
        });

        result += temp.value;
        error += temp.error;

        temp = gauss_kronrod_7_15_integral(&f, middle, upper_bound);
        intervals.push(Interval {
            a: middle,
            b: upper_bound,
            value: temp.value,
            error: temp.error,
        });

        result += temp.value;
        error += temp.error;
    }

    Measure {
        value: result,
        error,
    }
}
