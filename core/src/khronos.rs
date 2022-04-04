use std::fmt;

use common::exports::anyhow::{anyhow, Result};

///The real world time system.
pub struct NativeSystem(chrono::DateTime<chrono::Utc>);

///A general time trait  that conforms to the TimeSystem interface
pub trait TimeSystemTy {}
pub type CycleId = u32;
///A [Cycle] is implictely defined in terms of a Unit Cycle, which is the base unit of time
/// for a given system

///Represents a coefficient of another Cycle. Normal is a non fractional multiple of
/// of the cycle, inverted is a fractional multiple of the cycle.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Coef {
    Inverted(u32),
    Normal(u32),
}
impl Coef {
    pub fn mag(&self) -> u32 {
        match self {
            Coef::Inverted(mag) => *mag,
            Coef::Normal(mag) => *mag,
        }
    }
}
impl From<u32> for Coef {
    fn from(coef: u32) -> Self {
        Coef::Normal(coef)
    }
}
impl From<f32> for Coef {
    fn from(coef: f32) -> Self {
        if coef < 1.0 {
            Coef::Inverted((1.0 / (coef as f32)) as u32)
        } else {
            Coef::Normal((coef) as u32)
        }
    }
}
//impl comparison

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cycle {
    id: CycleId,
    pub name: String,
    pub description: String,
    pub coef: Coef,
    ///An optional name for each division of a cycle. This is similar to a week or a  month.
    pub div_names: Option<Vec<String>>,
    ///The cycle this cycle is relative to
    pub parent: CycleId,
}
//The unit cycle is the base unit of time for a given system, it is arbitraily defined as one ("unity")
pub struct UnitCycle {
    pub name: String,
    pub description: String,
}
impl Cycle {
    pub fn new(
        id: CycleId,
        name: &str,
        description: &str,
        coef: Coef,
        div_names: Option<Vec<String>>,
        parent: CycleId,
    ) -> Result<Self> {
        //make sure that the div names are the same length as the coef
        if let Some(ref div_names) = div_names {
            if (div_names.len() as u32) != coef.mag() {
                return Err(anyhow!(
                    "The number of div names must match the number of coefs"
                ));
            }
        }
        Ok(Cycle {
            id,
            name: String::from(name),
            description: String::from(description),
            coef,
            div_names,
            parent,
        })
    }
}

/// A [CycleMeasure] is a point in time represented as a multiple of a given [Cycle]
pub struct CycleMeasure {
    pub cycle: Cycle,
    pub moment: f64,
}
///Represents a single moment in time
pub struct DateTime {
    pub measures: Vec<CycleMeasure>,
}
impl DateTime {
    pub fn new(measures: Vec<CycleMeasure>) -> Self {
        DateTime { measures }
    }
    ///Returns a data time from a string of the form "CYCLEMEASURE1:CYCLEMEASURE2:CYCLEMEASURE3:..."
    /// Where a CYCLEMEASURE is just a number representing the coefficent of the measure
    /// So, if you had a system with a year, month, week and day,and wanted to calculate the first day of the third week of the second month and of 2020th year,
    /// Where a year is the unit cycle
    ///  you would use the string "2020:2:3:1"
    pub fn from_cycle_string(cycle_string: &str, system: &System) -> Self {
        let mut measures: Vec<CycleMeasure> = Vec::new();
        let mut cycle_string_iter = cycle_string.split(":").collect::<Vec<&str>>();
        //make sure the lengths are the same
        if cycle_string_iter.len() != system.cycles.len() + 1 {
            panic!(
                "The number of cycles in the string must match the number of cycles in the system"
            );
        }

        let cycle_iter = system.cycles.iter();
        //zip together
        for (cycle, coef) in cycle_iter.zip(cycle_string_iter) {
            let coef = coef.parse::<f32>().unwrap();
            let coef = Coef::from(coef);
            let measure = CycleMeasure {
                cycle: cycle.clone(),
                moment: coef.mag() as f64,
            };
            measures.push(measure);
        }

        DateTime { measures }
    }
}
//implement Display
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        for measure in &self.measures {
            string.push_str(&format!("{}:{}", measure.cycle.name, measure.moment));
        }
        write!(f, "{}", string)
    }
}
///An [Epoch] is a starting point for a long stretch of time (e.g. B.C.E, A.D. etc)
/// All [Epoch]s are implicitly defined in terms universal 0 Epoch
pub struct Epoch {
    pub name: String,
    //The number of Unit Cycles from the universal 0 Epoch
    pub delta_from_zero: f64,
}
///This is an attempt at a general time system.
pub struct System {
    pub unit_cycle: UnitCycle,
    pub cycles: Vec<Cycle>,
}

impl System {
    pub fn new() -> Self {
        System {
            unit_cycle: UnitCycle {
                name: String::from("unity"),
                description: String::from("The base unit of time"),
            },
            cycles: Vec::new(),
        }
    }
    pub fn from_cycles(unit: UnitCycle, cycles: impl IntoIterator<Item = Cycle>) -> Self {
        System {
            unit_cycle: unit,
            cycles: cycles.into_iter().collect(),
        }
    }
    ///For a cycle list to be valid, it must have the following properties:
    /// 1. None of the cycles should have a coef of less than 0
    /// 2. No cycle can have more than one cycles referencing it as a parent
    ///

    fn verify_cycles(cycles: Vec<Cycle>) -> Result<()> {
        if cycles.iter().any(|cycle| {
            cycle.parent != 0 && cycles.iter().filter(|c| c.id == cycle.parent).count() > 1
        }) {
            return Err(anyhow!("Cycle cannot be a child more than once"));
        }

        Ok(())
    }
    pub fn add_cycle(&mut self, cycle: Cycle) {
        self.cycles.push(cycle);
    }
    pub fn add_epoch(&mut self, epoch: Epoch) {
        self.epochs.push(epoch);
    }
    pub fn get_cycle(&self, cycle_id: CycleId) -> Option<&Cycle> {
        self.cycles.iter().find(|c| c.id == cycle_id)
    }
    pub fn get_epoch(&self, epoch_id: CycleId) -> Option<&Epoch> {
        todo!()
    }
    pub fn convert_to_cycle(&self, cycle_1: Cycle, cycle_2: Cycle) -> f64 {
        todo!()
    }
}

pub struct Duration {
    pub start: DateTime,
    pub end: DateTime,
}
impl Duration {
    pub fn new(start: DateTime, end: DateTime) -> Self {
        Duration { start, end }
    }
    pub fn get_duration(&self) -> f64 {
        todo!()
    }
}

pub struct TimeSystem<T>
where
    T: TimeSystemTy,
{
    pub system: T,
}
#[cfg(test)]
mod test_khronos {
    use super::*;

    #[test]
    fn test_basic() {
        let unit_cycle = UnitCycle {
            name: String::from("Year"),
            description: String::from("The base unit of time"),
        };
        let psuedo_month = Cycle::new(
            1,
            "Month",
            "A month",
            Coef::Inverted(12),
            Some(vec![
                "January".to_string(),
                "February".to_string(),
                "March".to_string(),
                "April".to_string(),
                "May".to_string(),
                "June".to_string(),
                "July".to_string(),
                "August".to_string(),
                "September".to_string(),
                "October".to_string(),
                "November".to_string(),
                "December".to_string(),
            ]),
            0,
        )
        .unwrap();
        let psuedo_week = Cycle::new(2, "Week", "A week", Coef::Inverted(4), None, 1).unwrap();
        let psuedo_day = Cycle::new(
            3,
            "Day",
            "A day",
            Coef::Inverted(7),
            Some(vec![
                "Sunday".to_string(),
                "Monday".to_string(),
                "Tuesday".to_string(),
                "Wednesday".to_string(),
                "Thursday".to_string(),
                "Friday".to_string(),
                "Saturday".to_string(),
            ]),
            2,
        )
        .unwrap();
        let psuedo_hour = Cycle::new(4, "Hour", "An hour", Coef::Inverted(24), None, 3).unwrap();
        let psuedo_minute =
            Cycle::new(5, "Minute", "A minute", Coef::Inverted(60), None, 4).unwrap();
        let psuedo_second =
            Cycle::new(6, "Second", "A second", Coef::Inverted(60), None, 5).unwrap();
        let system = System::from_cycles(
            unit_cycle,
            vec![
                psuedo_month,
                psuedo_week,
                psuedo_day,
                psuedo_hour,
                psuedo_minute,
                psuedo_second,
            ],
        );
        let date_time = DateTime::from_cycle_string("2022:1:3:5:13:54:54", &system);
    }
}
