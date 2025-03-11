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
        self.list.retain(|a| a.start >= point);
    }

    /// Returns previous activity end, strain, total strain and total break
    pub fn summary(
        &self,
        formula: &Formula,
        time: SystemTime,
    ) -> (Option<SystemTime>, Duration, Duration, Duration) {
        let (end, strain, total_strain, total_break) = self.list.iter().fold(
            (None, Duration::ZERO, Duration::ZERO, Duration::ZERO),
            |state, item| {
                let (prev_end, mut strain, mut total_strain, mut total_break) = state;
                let Activity { start, end } = item;
                if let Some(prev_end) = prev_end {
                    let to_break = formula.compute_break(strain, total_strain);
                    let actual_break = start.duration_since(prev_end).unwrap();
                    strain =
                        formula.compute_strain(to_break.saturating_sub(actual_break), total_strain);
                    total_break += to_break.min(actual_break);
                };
                let work = end.unwrap_or(time).duration_since(*start).unwrap();
                strain += work;
                total_strain += work;
                (*end, strain, total_strain, total_break)
            },
        );
        (
            end,
            if let Some(end) = end {
                formula.compute_strain(
                    formula
                        .compute_break(strain, total_strain)
                        .saturating_sub(time.duration_since(end).unwrap()),
                    total_strain,
                )
            } else {
                strain
            },
            total_strain,
            if let Some(end) = end {
                total_break
                    + formula
                        .compute_break(strain, total_strain)
                        .min(time.duration_since(end).unwrap())
            } else {
                total_break
            },
        )
    }

    pub fn switch(&mut self, time: SystemTime) {
        if let Some(last) = self.list.last_mut() {
            if last.end.is_some() {
                self.list.push(Activity {
                    start: time,
                    end: None,
                })
            } else {
                last.end = Some(time)
            }
        } else {
            self.list.push(Activity {
                start: time,
                end: None,
            })
        }
    }
}
