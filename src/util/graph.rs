use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait Graph {
    type Node;

    fn neighbors(&self, node: Self::Node) -> impl Iterator<Item = (Self::Node, usize)>;

    fn h(from: Self::Node, to: Self::Node) -> usize;

    fn a_star_distance(&self, from: Self::Node, to: Self::Node) -> Option<usize>
    where
        Self::Node: Copy + Hash + Eq,
    {
        let h = |from| Self::h(from, to);
        let mut open_set = BinaryHeap::new();
        open_set.push(MyNode {
            inner: from,
            score: h(from),
        });
        let mut g_score = HashMap::new();
        g_score.insert(from, 0);
        while let Some(current) = open_set.pop() {
            if current.inner == to {
                return Some(current.score);
            }

            let current = current.inner;
            for (neighbor, distance) in self.neighbors(current) {
                let tentative_g_score =
                    g_score.get(&current).copied().unwrap_or(usize::MAX) + distance;
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(usize::MAX) {
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push(MyNode {
                        inner: neighbor,
                        score: tentative_g_score + h(neighbor),
                    });
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct MyNode<T> {
    inner: T,
    score: usize,
}

impl<T> PartialEq for MyNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for MyNode<T> {}

impl<T> Ord for MyNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl<T> PartialOrd for MyNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
