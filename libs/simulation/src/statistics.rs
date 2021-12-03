use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub ga_birds: ga::Statistics,
    pub ga_eagles: ga::Statistics,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "generation {}:", self.generation)?;
        write!(
            f,
            "Birds: min[{:.2}] max[{:.2}] avg[{:.2}] median[{:.2}]",
            self.ga_birds.min_fitness(),
            self.ga_birds.max_fitness(),
            self.ga_birds.avg_fitness(),
            self.ga_birds.median_fitness()
        )?;
        write!(
            f,
            "Eagles: min[{:.2}] max[{:.2}] avg[{:.2}] median[{:.2}]",
            self.ga_eagles.min_fitness(),
            self.ga_eagles.max_fitness(),
            self.ga_eagles.avg_fitness(),
            self.ga_eagles.median_fitness()
        )
    }
}
