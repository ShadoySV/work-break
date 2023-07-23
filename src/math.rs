use std::time::Duration;

use serde::{Deserialize, Serialize};

pub struct Formula {
    a: f64,
    b: f64,
    c: f64,
}

impl Formula {
    pub fn new(a: &CoefficientA, b: &CoefficientB, c: &CoefficientC) -> Self {
        Self {
            a: a.0,
            b: b.0,
            c: c.0,
        }
    }
    pub fn compute_break(&self, strain: Duration) -> Duration {
        Duration::from_secs_f64(self.a * strain.as_secs_f64().powf(self.b) + self.c)
    }

    pub fn compute_strain(&self, work_break: Duration) -> Duration {
        Duration::from_secs_f64(
            (work_break
                .saturating_sub(Duration::from_secs_f64(self.c))
                .as_secs_f64()
                / self.a)
                .powf(self.b.recip()),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoefficientA(pub f64);

impl Default for CoefficientA {
    fn default() -> Self {
        Self(0.00147884224225867)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoefficientB(pub f64);

impl Default for CoefficientB {
    fn default() -> Self {
        Self(1.67098454496329)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoefficientC(pub f64);

impl Default for CoefficientC {
    fn default() -> Self {
        Self(0.)
    }
}
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Formula;

    #[test]
    fn pomodoro_and_efficiency_conversions() {
        let formula = Formula::new(
            &Default::default(),
            &Default::default(),
            &Default::default(),
        );
        assert_eq! {formula.compute_break(Duration::from_secs(25 * 60)).as_secs_f64().round(), 5. * 60.};
        assert_eq! {formula.compute_strain(Duration::from_secs(5 * 60)).as_secs_f64().round(), 25. * 60.};

        assert_eq! {formula.compute_break(Duration::from_secs(52 * 60)).as_secs_f64().round(), 17. * 60.};
        assert_eq! {formula.compute_strain(Duration::from_secs(17 * 60)).as_secs_f64().round(), 52. * 60.};
    }
}
