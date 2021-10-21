use std::{collections::HashSet, ops::Neg};

use num_traits::{Float, Num};

use rayon::prelude::*;

type F = f64;
type Vec2 = Vector<F, 2>;
type Vec2I = Vector<isize, 2>;

const SAMPLE_SIZE: usize = 100_000;

fn main() {
    // 2D random walk
    // a) on a lattice
    // b) on a lattice without consequent returns
    // c) self-avoiding walk

    let mut results: Vec<(usize, f64, f64)> = Vec::with_capacity(50);

    let mean_self_avoiding = (0..SAMPLE_SIZE).map(|_| (lattice_self_avoiding() as F)).sum::<F>() / SAMPLE_SIZE as F;

    println!("# Mean number of steps for 2D SAW = {} ({} samples)", mean_self_avoiding, SAMPLE_SIZE);

    println!("# steps\tlattice\tno_ret");

    (0..50)
        .into_par_iter()
        .map(|n| {
            let steps = (n + 1) * 20 - 10;
            let lattice: F = (0..SAMPLE_SIZE).map(|_| lattice_with_returns(steps)).sum::<F>() / (SAMPLE_SIZE as F);
            let lattice_no_ret: F = (0..SAMPLE_SIZE).map(|_| lattice_without_returns(steps)).sum::<F>() / (SAMPLE_SIZE as F);

            (steps, lattice, lattice_no_ret)
        })
        .collect_into_vec(&mut results);

    for (steps, lattice, lattice_no_ret) in results {
        println!("{}\t{}\t{}", steps, lattice, lattice_no_ret);
    }
}

fn usize_to_direction<T: Num + Neg<Output = T> + Copy>(dir: usize) -> Vector<T, 2> {
    match dir % 4 {
        0 => (T::one(), T::zero()),
        1 => (T::zero(), T::one()),
        2 => (-T::one(), T::zero()),
        _ => (T::zero(), -T::one()),
    }.into()
}

fn lattice_with_returns(steps: usize) -> f64 {
    let mut pos = Vec2::new([ 0.0, 0.0 ]);

    for _ in 0..steps {
        pos = pos + usize_to_direction(rand::random());
    }

    pos.len()
}

fn lattice_without_returns(steps: usize) -> f64 {
    let mut pos = Vec2::new([ 0.0, 0.0 ]);

    let mut prev_dir = 4;

    for _ in 0..steps {
        let dir: usize = {
            let mut dir;
            loop {
                dir = rand::random::<usize>() % 4;

                if (dir + 2) % 4 != prev_dir {
                    break;
                }
            }

            dir
        };

        prev_dir = dir;
        pos = pos + usize_to_direction(dir);
    }

    pos.len()
}

fn lattice_self_avoiding() -> usize {
    let mut pos = Vec2I::new([ 0, 0 ]);
    let mut steps = 0;

    let mut taken_fields: HashSet::<(isize, isize)> = HashSet::new();

    loop {
        let dir = {
            let mut pool: Vec<usize> = (0..4).collect();

            let mut dir = None;

            while pool.len() > 0 {
                let pool_idx = rand::random::<usize>() % pool.len();
                let temp_dir = usize_to_direction(pool[pool_idx]);

                if taken_fields.contains(&(pos[0] + temp_dir[0], pos[1] + temp_dir[1])) {
                    pool.remove(pool_idx);
                } else {
                    dir = Some(temp_dir);
                    break;
                }
            }

            dir
        };

        if let Some(dir) = dir {
            taken_fields.insert((pos[0], pos[1]));
            pos = pos + dir;
            steps += 1;
        } else {
            break;
        }
    }

    steps
}

struct Vector<T: Num, const N: usize>(pub [T; N]);

impl<T: Num + Copy, const N: usize> Vector<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self(data)
    }

    pub fn len_squared(&self) -> T {
        let mut acc = T::zero();

        self.0.iter().for_each(|v| acc = acc + *v * *v);

        acc
    }
}

impl<T: Float + Copy, const N: usize> Vector<T, N> {
    pub fn len(&self) -> T {
        self.len_squared().sqrt()
    }
}

impl<T: Num + Copy, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num + Copy, const N: usize> std::ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


impl<T: Num + Copy, const N: usize> std::ops::Add for Vector<T, N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..N {
            self.0[i] = self[i] + rhs[i];
        }

        self
    }
}

impl<T: Num + Copy> From<(T, T)> for Vector<T, 2> {
    fn from((x, y): (T, T)) -> Self {
        Vector::new([x, y])
    }
}
