use std::cmp::max;

pub struct intervalTree {
    elements: Vec<(isize, isize)>,
    size: usize,
}

impl intervalTree {
    pub fn new(size: usize) -> intervalTree
    {
       let mut size = find_next_pow(size)*2;
       let mut elements = vec![(0, 0); size];
       intervalTree {elements, size}
    }
    pub fn new_with_vector(el: Vec<isize>) -> intervalTree{
        let mut size = el.len();
        let mut size = find_next_pow(size)*2;
        let mut elements = vec![(0, 0); size];
        let mut inTree = intervalTree {elements, size};
        for i in 0..el.len() {
            inTree.add(i, i, el[i]);
        }
        inTree
    }
    pub fn add (&mut self, mut start: usize, mut end: usize, number: isize) {
        if start > end {
            panic!("The start of the interval is bigger than the end.")
        }
        start += self.size/2;
        end += self.size/2;
        if start != end {
            self.elements[start].0 += number;
            self.elements[end].0 += number;
            self.elements[start].1 += number;
            self.elements[end].1 += number;
            while start/2 != end/2 {
                if is_left_child (start) {
                    self.elements[start+1].0 += number;
                    self.elements[start+1].1 += number;
                }
                if is_right_child (end) {
                    self.elements[end-1].0 += number;
                    self.elements[end-1].1 += number;
                }
                start /= 2;
                end /= 2;
                self.elements[start].0 = max(self.elements[start*2].0, self.elements[start*2+1].0)+self.elements[start].1;
                self.elements[end].0 = max(self.elements[end*2].0, self.elements[end*2+1].0)+self.elements[end].1;
            }
            start /= 2;
            end /= 2;
        }
        else {
            self.elements[start].0 += number;
            self.elements[start].1 += number;
            start /= 2;
        }
        while start != 0 {
            self.elements[start].0 = max(self.elements[start*2].0, self.elements[start*2+1].0);
            start /= 2;
        }
    }
    
    pub fn find_max (&self, mut start: usize, mut end: usize) -> isize{
        start += self.size/2;
        end += self.size/2;
        let mut maxStart = self.elements[start].0;
        let mut maxEnd = self.elements[end].0;
        while start/2 != end/2 {
            if is_left_child (start) {
                maxStart = max(maxStart, self.elements[start+1].0);
            }
            if is_right_child(end) {
                maxEnd = max (maxEnd, self.elements[end-1].0);
            }
            start /= 2;
            end /= 2;
            maxStart += self.elements[start].1;
            maxEnd += self.elements[end].1;
        }
        start /= 2;
        end /= 2;
        maxStart = max (maxStart, maxEnd);
        println!("st: {}", maxStart);
        while start != 0 {
            maxStart += self.elements[start].1;
            start /= 2;
        }
        maxStart
    }

    fn print_tree (& self) {
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
