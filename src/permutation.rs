#[derive(Debug, Clone, PartialEq)]
pub struct Permutation {
    data: Vec<usize>
} 

impl Permutation {
    pub fn new(size: usize) -> Self {
        Permutation { data: (0..size).collect() }
    }

    pub fn get(&self, n: usize) -> usize {
        self.data[n]
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    pub fn data(&self) -> Vec<usize> {
        self.data.clone()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_identity(&self) -> bool {
        let identity: Vec<usize> = (0..self.size()).collect();
        self.data == identity
    }

    pub fn is_last(&self) -> bool {
        let last: Vec<usize> = (0..self.size()).rev().collect();
        self.data == last
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn inverse(&mut self) {
        let n: usize = self.size();
        let mut inverse: Vec<usize> = vec![0; n];

        for i in 0..n {
            inverse[self.data[i]] = i;
        }

        self.data = inverse
    }

    pub fn next(&mut self) {
        let n: usize = self.size();

        if !self.is_last() {
            // Find the rightmost index i such that permutation[i] < permutation[i + 1].
            let i = (0..n - 1)
                .rev()
                .find(|&i| self.data[i] < self.data[i + 1])
                .unwrap();

            // Find the rightmost element greater than permutation[i].
            let j = (i + 1..n)
                .rev()
                .find(|&j| self.data[j] > self.data[i])
                .unwrap();

            // Swap the pivot with that element.
            self.data.swap(i, j);

            // Reverse the suffix to obtain the next lexicographical permutation.
            self.data[i + 1..].reverse();
        }
    }

    pub fn next_cycling(&mut self) {
        let n: usize = self.size();

        if self.is_last() {
            self.data = (0..n).collect();
        } else {
            self.next();
        }
    }

    pub fn previous(&mut self) {
        let n: usize = self.size();

        if !self.is_identity() {
            // Find the rightmost index i such that permutation[i] < permutation[i + 1].
            let i = (0..n - 1)
                .rev()
                .find(|&i| self.data[i] > self.data[i + 1])
                .unwrap();

            // Find the rightmost element greater than permutation[i].
            let j = (i + 1..n)
                .rev()
                .find(|&j| self.data[j] < self.data[i])
                .unwrap();

            // Swap the pivot with that element.
            self.data.swap(i, j);

            // Reverse the suffix to obtain the next lexicographical permutation.
            self.data[i + 1..].reverse();
        }
    }

    pub fn previous_cycling(&mut self) {
        let n: usize = self.size();

        if self.is_identity() {
            self.data = (0..n).rev().collect();
        } else {
            self.previous();
        }
    }

    pub fn minimum_swap(&self) -> u32 {
        let n: usize = self.size();
        let mut visited: Vec<bool> = vec![false; n];
        let mut cycles: usize = 0;

        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut j = i;

                while !visited[j] {
                    visited[j] = true;
                    j = self.data[j];
                }
            }
        }

        (n - cycles) as u32
    }
}