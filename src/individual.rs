use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

use rand::Rng;

use crate::encodings::{BinaryCode, GrayCode, LAST_BIT_MASK};
use crate::probability::Probability;

const MIN_ENCODABLE: f64 = -32.768;
const MAX_ENCODABLE: f64 = 32.767;
const DISCRETIZATION_STEP: f64 = 0.001;
const CROSSING_OVER_MASK: u16 = 0b1111_1111_0000_0000;

enum CodeJoint {
    LeftRight,
    RightLeft,
}

trait Individual {
    fn value(code: GrayCode) -> f64 {
        BinaryCode::from(code).0 as f64 * DISCRETIZATION_STEP - MIN_ENCODABLE.abs()
    }

    fn fitness(code: GrayCode) -> f64 {
        let value = Self::value(code);

        value.sin() / value.powi(2)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NewbornIndividual {
    code: GrayCode,
    fitness: f64,
}

impl NewbornIndividual {
    pub fn build(value: f64) -> Option<Self> {
        if value.is_nan() || !(MIN_ENCODABLE..=MAX_ENCODABLE).contains(&value) {
            return None;
        }

        let result =
            BinaryCode(((value + MIN_ENCODABLE.abs()) / DISCRETIZATION_STEP).round() as u16);
        let code = result.into();

        Some(Self {
            code,
            fitness: Self::fitness(code),
        })
    }

    fn born(
        first: GrayCode,
        second: GrayCode,
        joint: CodeJoint,
        mutation_probability: Probability,
        survival_range: &RangeInclusive<f64>,
    ) -> Option<Self> {
        let code = GrayCode(match joint {
            CodeJoint::LeftRight => {
                (first.0 & CROSSING_OVER_MASK) | (second.0 & !CROSSING_OVER_MASK)
            }
            CodeJoint::RightLeft => {
                (first.0 & !CROSSING_OVER_MASK) | (second.0 & CROSSING_OVER_MASK)
            }
        });

        if !survival_range.contains(&Self::value(code)) {
            return None;
        }

        (Self {
            code,
            fitness: Self::fitness(code),
        })
        .mutate(mutation_probability, survival_range)
    }

    fn mutate(
        mut self,
        mutation_probability: Probability,
        survival_range: &RangeInclusive<f64>,
    ) -> Option<Self> {
        let mut rng = rand::rng();

        for i in 0..16 {
            let sample = rng.random_range(0.0..=1.0);

            if sample <= mutation_probability.value() {
                self.code.0 ^= LAST_BIT_MASK >> i;
            }
        }

        if !survival_range.contains(&Self::value(self.code)) {
            return None;
        }

        Some(self)
    }
}

impl Individual for NewbornIndividual {}

impl PartialEq for NewbornIndividual {
    fn eq(&self, other: &Self) -> bool {
        self.code.eq(&other.code)
    }
}

impl Eq for NewbornIndividual {}

impl Hash for NewbornIndividual {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl From<CompetingIndividual> for NewbornIndividual {
    fn from(individual: CompetingIndividual) -> Self {
        Self {
            code: individual.code,
            fitness: individual.fitness,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CompetingIndividual {
    code: GrayCode,
    fitness: f64,
}

impl CompetingIndividual {
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn value(&self) -> f64 {
        <Self as Individual>::value(self.code)
    }

    pub fn crossing_over(
        &self,
        other: &Self,
        survival_range: &RangeInclusive<f64>,
        mutation_probability: Probability,
    ) -> Vec<Self> {
        [CodeJoint::LeftRight, CodeJoint::RightLeft]
            .into_iter()
            .map(|joint| {
                NewbornIndividual::born(
                    self.code,
                    other.code,
                    joint,
                    mutation_probability,
                    survival_range,
                )
            })
            .filter(|child| child.is_some())
            .map(|child| child.unwrap().into())
            .collect()
    }
}

impl Individual for CompetingIndividual {}

impl From<NewbornIndividual> for CompetingIndividual {
    fn from(individual: NewbornIndividual) -> Self {
        Self {
            code: individual.code,
            fitness: individual.fitness,
        }
    }
}

impl PartialEq for CompetingIndividual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for CompetingIndividual {}

impl PartialOrd for CompetingIndividual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CompetingIndividual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_on_bounds() {
        assert_eq!(
            NewbornIndividual::build(MIN_ENCODABLE).unwrap().code,
            GrayCode::from(BinaryCode(0b0000_0000_0000_0000))
        );
        assert_eq!(
            NewbornIndividual::build(MAX_ENCODABLE).unwrap().code,
            GrayCode::from(BinaryCode(0b1111_1111_1111_1111))
        );
    }

    #[test]
    fn converts_to_competing_and_back() {
        let individual = NewbornIndividual::build(MIN_ENCODABLE).unwrap();
        let competing: CompetingIndividual = individual.into();
        let individual: NewbornIndividual = competing.into();

        assert_eq!(
            individual.code,
            GrayCode::from(BinaryCode(0b0000_0000_0000_0000))
        );
    }
}
