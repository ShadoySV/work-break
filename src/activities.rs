use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::math::Formula;

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub start: SystemTime,
    pub end: Option<SystemTime>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Activities {
    pub list: Vec<Activity>,
}

impl Activities {
    pub fn truncate_until(&mut self, point: SystemTime) {
        self.list.retain_mut(|a| {
            a.end
                .map(|e| {
                    let retain = e > point;
                    if retain && a.start < point {
                        a.start = point
                    };
                    retain
                })
                .unwrap_or(true)
        });
    }

    /// Returns end, strain and work
    pub fn work(
        &self,
        formula: &Formula,
        now: SystemTime,
    ) -> (Option<SystemTime>, Duration, Duration) {
        let (end, strain, work) =
            self.list
                .iter()
                .fold((None, Duration::ZERO, Duration::ZERO), |state, item| {
                    let (prev_end, mut strain, mut total_work) = state;
                    let Activity { start, end } = item;
                    if let Some(prev_end) = prev_end {
                        let rest = start.duration_since(prev_end).unwrap();
                        strain = formula
                            .compute_strain(formula.compute_break(strain).saturating_sub(rest));
                    };
                    let work = end.unwrap_or(now).duration_since(*start).unwrap();
                    strain += work;
                    total_work += work;

                    (*end, strain, total_work)
                });
        (
            end,
            if let Some(end) = end {
                formula.compute_strain(
                    formula
                        .compute_break(strain)
                        .saturating_sub(now.duration_since(end).unwrap()),
                )
            } else {
                strain
            },
            work,
        )
    }

    pub fn switch(&mut self, now: SystemTime) {
        if let Some(last) = self.list.last_mut() {
            if last.end.is_some() {
                self.list.push(Activity {
                    start: now,
                    end: None,
                })
            } else {
                last.end = Some(now)
            }
        } else {
            self.list.push(Activity {
                start: now,
                end: None,
            })
        }
    }
}
