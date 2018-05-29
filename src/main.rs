extern crate intervalTree;

fn main() {
    let mut in_tree = intervalTree::IntervalTree::new(8);
    /*inTree.add (0, 7, 5);
    inTree.print_tree();
    inTree.add (3, 5, 2);
    inTree.print_tree();
    inTree.add (5, 7, 3);
    println!("{}", inTree.find_max(6,7));*/
    in_tree.add(3,7,3);
    in_tree.add(4,4,5);
    in_tree.print_tree();
    println!("{}",in_tree.find_max(1,6));
}
