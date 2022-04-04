use common::exports::anyhow::{anyhow, Result};

///The real world time system.
pub struct NativeSystem(chrono::DateTime<chrono::Utc>);

///A general time trait representing that confirm to the TimeSystem interface
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
impl From<u32> for Coef {
    fn from(coef: u32) -> Self {
        Coef::Normal(coef)
    }
}
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
impl Cycle {
    pub fn new(
        name: &str,
        description: &str,
        coef: Coef,
        div_names: Option<Vec<String>>,
        parent: CycleId,
    ) -> Result<Self> {
        //make sure that the div names are the same length as the coef
        if let Some(div_names) = div_names {
            if div_names.len() != coef as usize {
                return Err(anyhow!(
                    "The number of div names must match the number of coefs"
                ));
            }
        }
        Ok(Cycle {
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
pub struct DateTime {}
impl DateTime {}
///An [Epoch] is a starting point for a long stretch of time (e.g. B.C.E, A.D. etc)
/// All [Epoch]s are implicitly defined in terms universal 0 Epoch
pub struct Epoch {
    pub name: String,
    //The number of Unit Cycles from the universal 0 Epoch
    pub delta_from_zero: f64,
}
///This is an attempt at a general time system.
pub struct System {
    pub cycles: Vec<Cycle>,
    pub unit_cycle_name: String,
    pub unit_cycle_description: String,
    pub epochs: Vec<Epoch>,
}

impl System {
    pub fn new() -> Self {
        System {
            cycles: Vec::new(),
            unit_cycle_name: String::new(),
            unit_cycle_description: String::new(),
            epochs: Vec::new(),
        }
    }
    pub fn from_cycles(cycles: impl IntoIterator<Item = Cycle>) -> Self {
        System {
            cycles: cycles.into_iter().collect(),
            unit_cycle_name: String::new(),
            unit_cycle_description: String::new(),
            epochs: Vec::new(),
        }
    }
    ///For a cycle list to be valid, it must have the following properties:
    /// 1. None of the cycles should have a coef of less than 0
    /// 2. No cycle can have more than one cycles referencing it as a parent
    ///

    fn verify_cycles(cycles: Vec<Cycle>) -> Result<()> {
        if cycles.iter().any(|cycle| cycle.coef < 0.0) {
            return Err(anyhow!("Cycle coefs cannot be less than 0"));
        }
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
        self.epochs.iter().find(|e| e.id == epoch_id)
    }
    pub fn convert_to_cycle(&self, cycle_1: Cycle, cycle_2: Cycle) -> f64 {}
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
        let psuedo_month = Cycle::new(
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
        let psuedo_week = Cycle::new("Week", "A week", Coef::Normal(7), None, 0).unwrap();
    }
}
