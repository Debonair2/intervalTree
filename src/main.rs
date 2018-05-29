extern crate intervalTree;

fn main() {
    let mut inTree = intervalTree::new(8);
    inTree.add (0, 7, 5);
    inTree.print_tree();
    inTree.add (3, 5, 2);
    inTree.print_tree();
    inTree.add (7, 7, 3);
    println!("{}", inTree.find_max(2,7));
}
