use std::collections::{BinaryHeap, HashSet};
use std::ops::RangeInclusive;

use rand::distr::{Distribution, Uniform};

use crate::individual::{CompetingIndividual, NewbornIndividual};
use crate::probability::Probability;

pub struct Population {
    competitors: BinaryHeap<CompetingIndividual>,
    size: usize,
    selection_intensity: f64,
    mutation_probability: Probability,
    survival_range: RangeInclusive<f64>,
}

impl Population {
    pub fn new(
        size: usize,
        selection_intensity: f64,
        mutation_probability: Probability,
        survival_range: RangeInclusive<f64>,
    ) -> Self {
        let mut individuals = HashSet::with_capacity(size);

        let mut rng = rand::rng();
        let distribution = Uniform::try_from(survival_range.clone()).unwrap();

        while individuals.len() < size {
            individuals.insert(NewbornIndividual::build(distribution.sample(&mut rng)).unwrap());
        }

        Self {
            competitors: individuals
                .into_iter()
                .map(|individual| individual.into())
                .collect(),
            size,
            selection_intensity,
            mutation_probability,
            survival_range,
        }
    }

    pub fn best(&self) -> (f64, f64) {
        let best_competitor = self.competitors.peek().unwrap();

        (best_competitor.value(), best_competitor.fitness())
    }

    pub fn create_generations(&mut self, count: usize) -> Vec<(f64, f64)> {
        let mut result = Vec::new();

        for _ in 0..count {
            self.next_generation();
            result.push(self.best());
        }

        result
    }

    fn next_generation(&mut self) {
        let mut selection_size = (self.selection_intensity * self.size as f64).ceil() as usize;
        let total_fitness = self.total_fitness();
        let mean_fitness = total_fitness / (self.size as f64);

        let mut rng = rand::rng();
        let selection_distribution = Uniform::try_from(0..=1).unwrap();

        let mut selected_candidates = Vec::with_capacity(selection_size);

        while !self.competitors.is_empty()
            && selection_size > 0
            && self.competitors.peek().unwrap().fitness() >= mean_fitness
        {
            if selection_distribution.sample(&mut rng) == 1 {
                selected_candidates.push(self.competitors.pop().unwrap());
            }

            selection_size -= 1;
        }

        let parents: Vec<_> = selected_candidates
            .iter()
            .step_by(2)
            .zip(selected_candidates.iter().skip(1).step_by(2).rev())
            .collect();

        parents
            .into_iter()
            .flat_map(|(first, second)| {
                first.crossing_over(second, &self.survival_range, self.mutation_probability)
            })
            .for_each(|child| self.competitors.push(child));
        selected_candidates
            .into_iter()
            .for_each(|candidate| self.competitors.push(candidate));

        self.shrink();
    }

    fn shrink(&mut self) {
        let mut retained = BinaryHeap::with_capacity(self.size);

        while !self.competitors.is_empty() && retained.len() < self.size {
            retained.push(self.competitors.pop().unwrap());
        }

        self.competitors = retained;
    }

    fn total_fitness(&self) -> f64 {
        self.competitors
            .iter()
            .map(|individual| individual.fitness())
            .sum()
    }
}
