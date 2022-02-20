
use rand::Rng;

use crate::chromosome::Chromosome;

fn invert_normalize<T, I: Iterator<Item=T>>(values: I) -> impl Iterator<Item=f64>
    where 
    T: Into<f64> + Clone,
    I: Clone {
    let coefficients = values
        .map(|v| -> f64 { 1_f64 / v.clone().into() });
    let sum: f64 = coefficients.clone().sum();
    coefficients.map(move |v| v / sum)
}

fn cascade_sum(vec: Vec<f64>) -> Vec<f64> {
    let mut sum: f64 = 0.;
    let mut result = Vec::with_capacity(vec.len());
    for i in vec {
        result.push(i + sum);
        sum += i;
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn invert_normalize_test() {
        use crate::selector::invert_normalize;
        assert_eq!(invert_normalize(vec![2, 4, 8, 8].into_iter()).collect::<Vec<_>>(), vec![0.5, 0.25, 0.125, 0.125]);
    }
    #[test]
    fn cascade_sum_test() {
        use crate::selector::cascade_sum;
        assert_eq!(cascade_sum(vec![0.5, 0.1, 0.4]), vec![0.5, 0.6, 1.0])
    }
}

pub trait Selector<T> {
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Chromosome<T>;
}

pub trait Fitness<T> {
    fn fitness_value(self: &Self, chromosome: &Chromosome<T>) -> T;
    fn selector(self: &Self) -> FitnessSelector<Self> where Self: Sized { FitnessSelector { fitness: self } }
}

pub struct FitnessSelector<'a, F> {
    fitness: &'a F
}

impl <'a, F> FitnessSelector<'a, F> {
    pub fn from_fitness(f: &'a F) -> Self { FitnessSelector { fitness: f } }
}

impl<'a, T: Into<f64> + Clone, F: Fitness<T>> Selector<T> for FitnessSelector<'a, F> {
    fn select_chromosome<R : rand::RngCore>(self: &Self, chromosomes: &Vec<Chromosome<T>>, rng: &mut R) -> Chromosome<T> {
        let fitness = chromosomes.iter().map(|c| self.fitness.fitness_value(c));
        let nfv: Vec<_> =  cascade_sum(invert_normalize(fitness)
            .map(|x| x.abs()).collect());
        
        for i in 0..nfv.len() - 1 {
            let p: f64 = rng.gen();
            if p < nfv[i] && if i > 0 { p > nfv[i - 1] } else { true } {
                return chromosomes[i].clone();
            }
        }
        chromosomes.last().unwrap().clone()
    }

}            
