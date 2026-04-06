use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::Distribution;
use rand_distr::Exp;
use rand_distr::LogNormal;
use rand_distr::Pareto;
use rand_distr::Uniform;

const SEED: u64 = 42;

pub fn sequential(n: usize) -> Vec<u64> {
    (1..=n as u64).collect()
}

pub fn uniform(n: usize, max: u64) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Uniform::new(1u64, max).unwrap();
    (0..n).map(|_| dist.sample(&mut rng)).collect()
}

pub fn log_normal_api(n: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = LogNormal::new(6.0, 0.5).unwrap();
    (0..n).map(|_| dist.sample(&mut rng) as u64).collect()
}

pub fn bimodal(n: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let fast = LogNormal::new(5.0, 0.3).unwrap();
    let slow = LogNormal::new(8.0, 0.5).unwrap();
    let coin = Uniform::new(0.0f64, 1.0).unwrap();
    (0..n)
        .map(|_| {
            if coin.sample(&mut rng) < 0.9 {
                fast.sample(&mut rng) as u64
            } else {
                slow.sample(&mut rng) as u64
            }
        })
        .collect()
}

pub fn exponential(n: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Exp::new(0.001).unwrap();
    (0..n).map(|_| dist.sample(&mut rng) as u64).collect()
}

pub fn pareto_heavy(n: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Pareto::new(1.0, 1.5).unwrap();
    (0..n).map(|_| dist.sample(&mut rng) as u64).collect()
}
