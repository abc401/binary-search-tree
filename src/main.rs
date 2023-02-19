mod bst;

use bst::BST;

fn main() {
    let values = vec![5, 6, 3, 7, 37, 7, 24, 753, 34, 7, 3, 573423];
    let tree = BST::from(values);
    println!("####################      Displaying The Tree     ######################");
    tree.display();

    println!("####################      Searching for 753     ######################");
    if let Some(tree) = tree.search(753) {
        tree.display()
    } else {
        println!("Could not find");
    }

    println!("####################      Searching for 752     ######################");
    if let Some(tree) = tree.search(752) {
        tree.display()
    } else {
        println!("Could not find");
    }
}
