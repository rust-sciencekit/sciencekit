use std::collections::BinaryHeap;

use crate::{integral::adaptive::Interval, measure::Measure};

pub fn trapezoidal<F>(f: F, a: f64, b: f64, abs_error: f64, rel_error: f64) -> Measure
where
    F: Fn(f64) -> f64,
{
    let mut result: f64 = 0.0;
    let mut error: f64 = 0.0;

    let mut intervals: BinaryHeap<Interval> = BinaryHeap::new();

    let mut temp: Measure = crate::integral::non_adaptive::trapezoidal(&f, a, (b + a) / 2.0, 10);
    result += temp.value;
    error += temp.error;
    intervals.push(Interval {
        a,
        b: (b + a) / 2.0,
        value: temp.value,
        error: temp.error,
    });

    temp = crate::integral::non_adaptive::trapezoidal(&f, (b + a) / 2.0, b, 10);
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

        temp = crate::integral::non_adaptive::trapezoidal(&f, lower_bound, middle, 10);
        intervals.push(Interval {
            a: lower_bound,
            b: middle,
            value: temp.value,
            error: temp.error,
        });

        result += temp.value;
        error += temp.error;

        temp = crate::integral::non_adaptive::trapezoidal(&f, middle, upper_bound, 10);
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
