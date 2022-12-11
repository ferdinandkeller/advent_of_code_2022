mod tree;

use std::fs;
use tree::{TreeVisibility, Tree};
use thousands::Separable;

/// Parses the input into a forest of trees.
fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    // create a forest of trees
    let mut forest = vec![];

    // for each line in the input
    for line in input.lines() {
        // create a row of trees
        let mut forest_row = vec![];

        // convert the line to a vector of chars
        let chars: Vec<char> = line.chars().collect();
        
        // for each char in the line
        for char in chars {
            // parse the char to a usize
            let tree_height: usize = char.to_string().parse().unwrap();

            // add a tree to the row
            forest_row.push(Tree::new(tree_height));
        }

        // add the row to the forest
        forest.push(forest_row);
    }

    // return the forest
    return forest;
}

/// Updates the visibility of the trees in the forest.
fn update_visibility(forest: &mut Vec<Vec<Tree>>) {
    // find the forest dimensions
    let forest_height = forest.len();
    let forest_width = forest[0].len();

    // set the visibility of the trees on the edges of the forest to visible
    for row in 0..forest_height {
        // east trees
        forest[row][0].visibility = TreeVisibility::Visible;
        // west trees
        forest[row][forest_width - 1].visibility = TreeVisibility::Visible;
    }
    for col in 0..forest_width {
        // north trees
        forest[0][col].visibility = TreeVisibility::Visible;
        // south trees
        forest[forest_height - 1][col].visibility = TreeVisibility::Visible;
    }

    // for each row in the forest
    for row in 1..forest_height - 1 {
        // looking west
        let mut heighest_tree = forest[row][0];
        let mut current_view_distance = [1; 10];

        for col in 1..forest_width {
            // find the currently selected tree
            let tree = &mut forest[row][col];

            // if the tree is higher than the heighest tree
            // then it is visible
            if *tree > heighest_tree {
                tree.visibility = TreeVisibility::Visible;
                heighest_tree = *tree;
            }

            // update the view distance of the tree
            tree.west_view_distance = current_view_distance[tree.height];

            // all the trees below or equal to the current tree won't be able to see further
            for height in 0..=tree.height {
                current_view_distance[height] = 1;
            }
            for height in tree.height + 1..10 {
                current_view_distance[height] += 1;
            }
        }

        // looking east
        let mut heighest_tree = forest[row][forest_width - 1];
        let mut current_view_distance = [1; 10];

        for col in (0..forest_width - 1).rev() {
            // find the currently selected tree
            let tree = &mut forest[row][col];

            // if the tree is higher than the heighest tree
            // then it is visible
            if *tree > heighest_tree {
                tree.visibility = TreeVisibility::Visible;
                heighest_tree = *tree;
            }

            // update the view distance of the tree
            tree.east_view_distance = current_view_distance[tree.height];

            // all the trees below or equal to the current tree won't be able to see further
            for height in 0..=tree.height {
                current_view_distance[height] = 1;
            }
            for height in tree.height + 1..10 {
                current_view_distance[height] += 1;
            }
        }
    }

    for col in 1..forest_width - 1 {
        // looking north
        let mut heighest_tree = forest[0][col];
        let mut current_view_distance = [1; 10];

        for row in 1..forest_height {
            // find the currently selected tree
            let tree = &mut forest[row][col];

            // if the tree is higher than the heighest tree
            // then it is visible
            if *tree > heighest_tree {
                tree.visibility = TreeVisibility::Visible;
                heighest_tree = *tree;
            }

            // update the view distance of the tree
            tree.north_view_distance = current_view_distance[tree.height];

            // all the trees below or equal to the current tree won't be able to see further
            for height in 0..=tree.height {
                current_view_distance[height] = 1;
            }
            for height in tree.height + 1..10 {
                current_view_distance[height] += 1;
            }
        }

        // looking south
        let mut heighest_tree = forest[forest_height - 1][col];
        let mut current_view_distance = [1; 10];

        for row in (0..forest_height - 1).rev() {
            // find the currently selected tree
            let tree = &mut forest[row][col];

            // if the tree is higher than the heighest tree
            // then it is visible
            if *tree > heighest_tree {
                tree.visibility = TreeVisibility::Visible;
                heighest_tree = *tree;
            }

            // update the view distance of the tree
            tree.south_view_distance = current_view_distance[tree.height];

            // all the trees below or equal to the current tree won't be able to see further
            for height in 0..=tree.height {
                current_view_distance[height] = 1;
            }
            for height in tree.height + 1..10 {
                current_view_distance[height] += 1;
            }
        }
    }
}

/// The goal of this challenge is to find the number of visible trees and the highest scenic score in a given forest.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_08.txt").expect("Unable to read the input file");

    // parse the input into a forest of trees
    let mut forest = parse_input(&input);

    // update the visibility of the trees
    update_visibility(&mut forest);

    // find the forest dimensions
    let forest_height = forest.len();
    let forest_width = forest[0].len();

    // count the number of visible trees and find the highest scenic score
    let mut number_of_visible_trees: usize = 0;
    let mut highest_scenic_score: usize = 0;

    for row in 0..forest_height {
        for col in 0..forest_width {
            let tree = &forest[row][col];

            if tree.visibility == TreeVisibility::Visible {
                number_of_visible_trees += 1;
            }

            let scenic_score = tree.scenic_score();
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!(
        "In total, there are {number_of_visible_trees} visible trees.",
        number_of_visible_trees = number_of_visible_trees.separate_with_commas()
    );
    println!(
        "The highest scenic score possible is {highest_scenic_score}.",
        highest_scenic_score = highest_scenic_score.separate_with_commas()
    );
}
