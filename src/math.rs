use std::time::Duration;

use serde::{Deserialize, Serialize};

pub struct Formula {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Formula {
    pub fn new(a: &CoefficientA, b: &CoefficientB, c: &CoefficientC, d: &CoefficientD) -> Self {
        Self {
            a: a.0,
            b: b.0,
            c: c.0,
            d: d.0,
        }
    }
    pub fn compute_break(&self, strain: Duration, total_work: Duration) -> Duration {
        Duration::from_secs_f64(
            self.a
                * strain
                    .as_secs_f64()
                    .powf(self.b + self.d * total_work.as_secs_f64())
                + self.c,
        )
    }

    pub fn compute_strain(&self, work_break: Duration, total_work: Duration) -> Duration {
        Duration::from_secs_f64(
            (work_break
                .saturating_sub(Duration::from_secs_f64(self.c))
                .as_secs_f64()
                / self.a)
                .powf((self.b + self.d * total_work.as_secs_f64()).recip()),
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
#[derive(Serialize, Deserialize)]
pub struct CoefficientD(pub f64);

impl Default for CoefficientD {
    fn default() -> Self {
        Self(0.)
    }
}
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::math::CoefficientD;

    use super::Formula;

    const POMODORO_WORK: f64 = 25. * 60.;
    const POMODORO_BREAK: f64 = 5. * 60.;
    const EFFICIENCY_WORK: f64 = 52. * 60.;
    const EFFICIENCY_BREAK: f64 = 17. * 60.;

    #[test]
    fn pomodoro_and_efficiency_conversions() {
        let formula = Formula::new(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            &Default::default(),
        );

        assert_eq! {formula.compute_break(Duration::from_secs_f64(POMODORO_WORK), Duration::ZERO).as_secs_f64().round(), POMODORO_BREAK};
        assert_eq! {formula.compute_strain(Duration::from_secs_f64(POMODORO_BREAK), Duration::ZERO).as_secs_f64().round(), POMODORO_WORK};

        assert_eq! {formula.compute_break(Duration::from_secs_f64(EFFICIENCY_WORK), Duration::ZERO).as_secs_f64().round(), EFFICIENCY_BREAK};
        assert_eq! {formula.compute_strain(Duration::from_secs_f64(EFFICIENCY_BREAK), Duration::ZERO).as_secs_f64().round(), EFFICIENCY_WORK};
    }

    #[test]
    fn four_hours_with_one_to_one_ratio() {
        let formula = Formula::new(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            &CoefficientD(0.00001528),
        );

        const FOUR_HOURS: f64 = 4. * 60. * 60.;
        const ALMOST_HUNDRED_MINUTES: f64 = 5990.;

        assert_eq! {formula.compute_break(Duration::from_secs_f64(POMODORO_WORK), Duration::from_secs_f64(FOUR_HOURS)).as_secs_f64().round(), POMODORO_WORK};
        assert_eq! {formula.compute_strain(Duration::from_secs_f64(POMODORO_WORK), Duration::from_secs_f64(FOUR_HOURS)).as_secs_f64().round(), POMODORO_WORK};

        assert_eq! {formula.compute_break(Duration::from_secs_f64(EFFICIENCY_WORK), Duration::from_secs_f64(FOUR_HOURS)).as_secs_f64().round(), ALMOST_HUNDRED_MINUTES};
        assert_eq! {formula.compute_strain(Duration::from_secs_f64(ALMOST_HUNDRED_MINUTES), Duration::from_secs_f64(FOUR_HOURS)).as_secs_f64().round(), EFFICIENCY_WORK};
    }
}
