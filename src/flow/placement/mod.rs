use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::iter::{Cycle, Iterator};

use flow::prelude::{WorkerEndpoint, WorkerIdentifier};
use flow::domain;

pub trait DomainPlacementStrategy {
    fn place_domain(&mut self, d: &domain::Index, s: usize) -> Option<WorkerIdentifier>;
}

pub(crate) struct RoundRobinPlacer<'a> {
    iter: Cycle<Iter<'a, WorkerIdentifier, WorkerEndpoint>>,
}

impl<'a> RoundRobinPlacer<'a> {
    pub fn new(workers: &'a HashMap<WorkerIdentifier, WorkerEndpoint>) -> Self {
        RoundRobinPlacer {
            iter: workers.iter().cycle(),
        }
    }
}

impl<'a> DomainPlacementStrategy for RoundRobinPlacer<'a> {
    fn place_domain(&mut self, _: &domain::Index, _: usize) -> Option<WorkerIdentifier> {
        self.iter.next().map(|ref w| w.0.clone())
    }
}
