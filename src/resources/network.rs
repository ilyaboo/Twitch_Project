use crate::resources::graph::Graph;
use crate::resources::matrix::Matrix;

use std::fs::File;
use std::io::prelude::*;

pub struct Network<'a> {
    country: &'a str,
    graph: Graph,
    matrix: Matrix,
}

impl Network<'_> {
    pub fn new(country: &str, graph: Graph, matrix: Matrix) -> Network {
        // method that creates and instance of the class
        return Network {
            country: country,
            graph: graph,
            matrix: matrix,
        };
    }

    pub fn graph_info(&self) {
        // helper method that prints all information about the graph
        self.graph.info();
    }

    pub fn matrix_info(&self) {
        // helper method that prints all information about the matrix
        self.matrix.info();
    }

    pub fn get_info(&self) {
        // function that prints all information about the country's network
        println!("{}", self.country);
        self.graph_info();
        self.matrix_info();
    }
}

pub fn read_file<'a>(path: &'a str, country: &'a str) -> Network {
    // function that reads the graph from the file
    let mut vertices_num: usize = 0;
    let mut edges_num: usize = 0;
    let mut edges_vector: Vec<(usize, usize)> = Vec::new();

    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x: usize = v[0].parse::<usize>().unwrap();
        let y: usize = v[1].parse::<usize>().unwrap();
        edges_vector.push((x, y));
        edges_num += 1;
        if x > vertices_num {
            vertices_num = x;
        } else if y > vertices_num {
            vertices_num = y;
        }
    }
    vertices_num += 1;
    let graph: Graph = Graph::new(vertices_num, edges_num, edges_vector);
    let matrix: Matrix = Matrix::new(&graph);
    return Network::new(country, graph, matrix);
}

pub fn graphs_info(countries: &Vec<Network>) {
    // prints information about all the graphs in the vector
    println!("=========================================================");
    println!("GRAPHS INFO");

    for country in countries {
        println!();
        println!("{}", country.country);
        country.graph_info();
    }
    println!("=========================================================\n\n\n");
}

pub fn matrices_info(countries: &Vec<Network>) {
    // prints information about all the matrices in the vector
    println!("=========================================================");
    println!("MATRICES INFO");

    for country in countries {
        println!();
        println!("{}", country.country);
        country.matrix_info();
    }
    println!("=========================================================\n\n\n");
}

pub fn networks_info(countries: &Vec<Network>) {
    // prints information about all the networks in the vector
    println!("=========================================================");
    println!("NETWORKS INFO");

    for country in countries {
        println!();
        country.get_info();
    }
    println!("=========================================================\n\n\n");
}

pub fn sort_by_friend_nums(countries: &Vec<Network>) {
    // prints countries in descending order of average number of friends
    println!("=========================================================");
    println!("AVERAGE NUMBER OF FRIENDS IN DESCENDING ORDER\n");
    let mut list: Vec<(i64, &str, f64)> = Vec::new();
    for i in 0..countries.len() {
        list.push((
            (countries[i].graph.avg_num_friends() * 1000000.0).round() as i64,
            countries[i].country,
            countries[i].graph.avg_num_friends(),
        ));
    }
    list.sort_by(|a, b| b.0.cmp(&a.0)); // decreasing order
    for i in 0..list.len() {
        println!("{}: {} friends", list[i].1, list[i].2);
    }
    println!("=========================================================\n\n\n");
}

pub fn sort_by_density(countries: &Vec<Network>) {
    // prints countries in descending order of density of a graph
    println!("=========================================================");
    println!("GRAPH DENSITY IN DESCENDING ORDER\n");
    let mut list: Vec<(i64, &str, f64)> = Vec::new();
    for i in 0..countries.len() {
        list.push((
            (countries[i].graph.density() * 1000000.0).round() as i64,
            countries[i].country,
            countries[i].graph.density(),
        ));
    }
    list.sort_by(|a, b| b.0.cmp(&a.0)); // decreasing order
    for i in 0..list.len() {
        println!("{}: {}", list[i].1, list[i].2);
    }
    println!("=========================================================\n\n\n");
}

pub fn sort_by_cluestering_coefficient(countries: &Vec<Network>) {
    // prints countries in descending order of clustering coefficient
    println!("=========================================================");
    println!("CLUSTERING COEFFICIENT IN DESCENDING ORDER\n");
    let mut list: Vec<(i64, &str, f64)> = Vec::new();
    for i in 0..countries.len() {
        list.push((
            (countries[i].matrix.clustering_coefficient() * 1000000.0).round() as i64,
            countries[i].country,
            countries[i].matrix.clustering_coefficient(),
        ));
    }
    list.sort_by(|a, b| b.0.cmp(&a.0)); // decreasing order
    for i in 0..list.len() {
        println!("{}: {}", list[i].1, list[i].2);
    }
    println!("=========================================================\n\n\n");
}

pub fn sort_by_friends_friends_prob(countries: &Vec<Network>) {
    // prints countries in descending order of friends' friends probability
    println!("=========================================================");
    println!("AVERAGE PROBABILITY THAT A FRIEND OF A FRIEND\nOF A USER IS ALSO USER'S FRIEND IN DESCENDING ORDER\n");
    let mut list: Vec<(i64, &str, f64)> = Vec::new();
    for i in 0..countries.len() {
        list.push((
            (countries[i].matrix.friends_of_friends_prob() * 1000000.0).round() as i64,
            countries[i].country,
            countries[i].matrix.friends_of_friends_prob(),
        ));
    }
    list.sort_by(|a, b| b.0.cmp(&a.0)); // decreasing order
    for i in 0..list.len() {
        println!("{}: {}", list[i].1, list[i].2);
    }
    println!("=========================================================\n\n\n");
}
