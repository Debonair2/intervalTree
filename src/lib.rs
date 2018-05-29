use std::cmp::max;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    max: isize,
    add: isize
}

pub struct IntervalTree {
    elements: Vec<Node>,
    size: usize,
    real_size: usize
}

impl IntervalTree {
    pub fn new(size: usize) -> IntervalTree
    {
        let real_size = size;
        let size = find_next_pow(size)*2;
        let elements = vec![Node {max: 0, add: 0}; size];
        IntervalTree {elements, size, real_size}
    }
    pub fn new_with_vector(el: Vec<isize>) -> IntervalTree{
        let real_size = el.len();
        let size = el.len();
        let size = find_next_pow(size)*2;
        let elements = vec![Node {max: 0, add: 0}; size];
        let mut in_tree = IntervalTree {elements, size, real_size};
        for i in 0..el.len() {
            in_tree.add(i, i, el[i]);
        }
        in_tree
    }
    pub fn add (&mut self, mut start: usize, mut end: usize, number: isize) {
        if start > end {
            panic!("The start of the interval is bigger than the end.")
        }
        if end >= self.real_size {
            panic!("You specified wrong query size.")
        }
        start += self.size/2;
        end += self.size/2;
        if start != end {
            self.elements[start].max += number;
            self.elements[end].max += number;
            self.elements[start].add += number;
            self.elements[end].add += number;
            while start/2 != end/2 {
                if is_left_child (start) {
                    self.elements[start+1].max += number;
                    self.elements[start+1].add += number;
                }
                if is_right_child (end) {
                    self.elements[end-1].max += number;
                    self.elements[end-1].add += number;
                }
                start /= 2;
                end /= 2;
                self.elements[start].max = max(self.elements[start*2].max, self.elements[start*2+1].max)+self.elements[start].add;
                self.elements[end].max = max(self.elements[end*2].max, self.elements[end*2+1].max)+self.elements[end].add;
            }
            start /= 2;
        }
        else {
            self.elements[start].max += number;
            self.elements[start].add += number;
            start /= 2;
        }
        while start != 0 {
            self.elements[start].max = max(self.elements[start*2].max, self.elements[start*2+1].max)+self.elements[start].add;
            start /= 2;
        }
    }
    
    pub fn find_max (&self, mut start: usize, mut end: usize) -> isize{
        if start > end {
            panic!("The start of the interval is bigger than the end.")
        }
        if end >= self.real_size {
            panic!("End of the interval is bigger than interval.")
        }
        start += self.size/2;
        end += self.size/2;
        let mut max_start = self.elements[start].max;
        let mut max_end = self.elements[end].max;
        while start/2 != end/2 {
            if is_left_child (start) {
                max_start = max(max_start, self.elements[start+1].max);
            }
            if is_right_child(end) {
                max_end = max (max_end, self.elements[end-1].max);
            }
            start /= 2;
            end /= 2;
            max_start += self.elements[start].add;
            max_end += self.elements[end].add;
        }
        start /= 2;
        max_start = max (max_start, max_end);
        println!("st: {}", max_start);
        while start != 0 {
            max_start += self.elements[start].add;
            start /= 2;
        }
        max_start
    }

    pub fn print_tree (& self) {
        let mut counter = 1;
        while counter < self.size {
            for i in 0..counter {
                print!("{:?} ", self.elements[counter+i]);
            }
            print!("\n");
            counter*=2;
        }
    }

}
//insanely bad internet connection so using personal implementation of random
pub fn random() -> usize
{
    let mut number: usize = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize;
    number *= 15;
    number %= 12457;
    number ^= 7;
    number %= 1001;
    number *= 1003;
    number
}


fn is_left_child (number: usize) -> bool {
    match number%2 {
        0 => true,
        1 => false,
        _ => false
    }
}

fn is_right_child (number: usize) ->bool {
    let number = number%2;
    match number {
        0 => false,
        1 => true,
        _ => false
    }
}

fn find_next_pow (number: usize) -> usize {
    let mut result = 1;
    while result < number {
        result *= 2;
    }
    result
}


struct BruteTree {
    elements: Vec<isize>
}

impl BruteTree {
    fn new (size: usize) -> BruteTree {
        let elements = vec![0; size];
        BruteTree {elements}
    }
    fn add (&mut self, start: usize, end: usize, number: isize) {
        for i in start..end+1 {
            self.elements[i] += number;
        }

    }
    fn find_max (&self, start: usize, end: usize) -> isize
    {
        let mut max = self.elements[start];
        for i in start..end+1 {
            if self.elements[i] > max {
                max = self.elements[i];
            }
        }
        max
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_interval_tree_correctness() {
        let size = 8;
        let mut in_tree = IntervalTree::new(size);
        let mut br_tree = BruteTree::new(size);
        for _i in 0..10000 {
            let st = random()%size;
            let end = random()%(size-st) + st;
            let number = (random()%10) as isize;
            println!("Add {} {} {}", st, end, number);
            in_tree.add(st, end, number);
            br_tree.add(st, end, number);
            let st = random()%size;
            let end = random()%(size-st) + st;
            println!("Check {} {}", st, end);
            assert_eq!(in_tree.find_max(st, end), br_tree.find_max(st, end));
        }

    }

}