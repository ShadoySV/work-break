use std::time::Duration;

// break = a * (work ^ b)
const A: f64 = 0.00147884224225867;
const B: f64 = 1.67098454496329;

pub fn work_break(work: Duration) -> Duration {
    Duration::from_secs_f64(A * work.as_secs_f64().powf(B))
}

pub fn work(workbreak: Duration) -> Duration {
    Duration::from_secs_f64((workbreak.as_secs_f64() / A).powf(B.recip()))
}

#[test]
fn calculate() {
    assert_eq! {work_break(Duration::from_secs(25 * 60)).as_secs_f64().round(), 5. * 60.};
    assert_eq! {work(Duration::from_secs(5 * 60)).as_secs_f64().round(), 25. * 60.};

    assert_eq! {work_break(Duration::from_secs(52 * 60)).as_secs_f64().round(), 17. * 60.};
    assert_eq! {work(Duration::from_secs(17 * 60)).as_secs_f64().round(), 52. * 60.};
}
