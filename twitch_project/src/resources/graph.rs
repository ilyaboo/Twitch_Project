pub struct Graph {
    // creating struct for the graph
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new(num_vertices: usize, num_edges: usize, edges: Vec<(usize, usize)>) -> Graph {
        // implementing new method that creates an instance of the graph object
        return Graph {
            num_vertices: num_vertices,
            num_edges: num_edges,
            edges: edges,
        };
    }

    pub fn num_vertices(&self) -> usize {
        // helper method that returns the number of vertices and keeps the field private
        return self.num_vertices;
    }

    pub fn num_edges(&self) -> usize {
        // helper method that returns the number of edges and keeps the field private
        return self.num_edges;
    }

    pub fn edges(&self) -> &Vec<(usize, usize)> {
        // helper method that returns the edges vector and keeps the field private
        return &self.edges;
    }

    pub fn info(&self) {
        // prints the information abou the graph
        println!("number of users: {}", self.num_vertices);
        println!("number of friend links: {}", self.num_edges);
        println!("connections:");
        for i in 0..5 {
            println!("   {} and {}", self.edges[i].0, self.edges[i].1);
        }
        println!("   ...");
        println!(
            "   {} and {}",
            self.edges[self.edges.len() - 1].0,
            self.edges[self.edges.len() - 1].1
        );
        println!(
            "average number of friends of a user: {}",
            self.avg_num_friends()
        );
        println!("graph density: {}", self.density());
    }

    pub fn avg_num_friends(&self) -> f64 {
        // calculates the average number of friends of the users in the graph
        return (self.num_edges as f64) * 2.0 / (self.num_vertices as f64);
    }

    pub fn density(&self) -> f64 {
        // calculates density of the graph
        return 2.0 * (self.num_edges as f64)
            / ((self.num_vertices * (self.num_vertices - 1)) as f64);
    }
}
