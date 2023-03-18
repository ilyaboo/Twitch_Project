use crate::resources::graph::Graph;
pub struct Matrix {
    // creating struct for the matrix
    num_vertices: usize,
    num_edges: usize,
    matrix: Vec<Vec<usize>>,
}

impl Matrix {
    pub fn new(graph: &Graph) -> Matrix {
        // implementing new method that creates an instance Matrix using the graph
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; graph.num_vertices()]; graph.num_vertices()];
        for pair in graph.edges() {
            matrix[pair.0][pair.1] = 1;
            matrix[pair.1][pair.0] = 1;
        }
        return Matrix {
            num_vertices: graph.num_vertices(),
            num_edges: graph.num_vertices(),
            matrix: matrix,
        };
    }

    pub fn print(&self) {
        // helper method that prints the matrix
        for ln in 0..self.matrix.len() {
            for col in 0..self.matrix[0].len() {
                print!("{} ", self.matrix[ln][col]);
            }
            println!();
        }
    }

    pub fn info(&self) {
        // method that prints info about the matrix
        println!("number of triangles: {}", self.num_triangles());
        //println!("number of triangles new: {}", self.num_triangles_new());
        println!("number of triplets: {}", self.num_triplets());
        //println!("number of triplets new: {}", self.num_triplets_new());
        println!("clustering coefficient: {}", self.clustering_coefficient());
        println!(
            "friends' friends probability: {}",
            self.friends_of_friends_prob()
        );
    }

    fn num_triangles(&self) -> usize {
        // return the number of triangles in the network
        let mut num_triangles: usize = 0;
        for one in 0..self.matrix.len() {
            for two in (one + 1)..self.matrix.len() {
                for three in (two + 1)..self.matrix.len() {
                    if (self.matrix[one][two] == 1)
                        && (self.matrix[two][three] == 1)
                        && (self.matrix[one][three] == 1)
                    {
                        num_triangles += 1;
                    }
                }
            }
        }
        return num_triangles;
    }

    fn num_triplets(&self) -> usize {
        // return a number of triplets in the network
        // triplet is just 3 vertices that are interconnected
        return (self.num_vertices * (self.num_vertices - 1) * (self.num_vertices - 2)) / 6;
    }

    pub fn clustering_coefficient(&self) -> f64 {
        // calculates transitivity of the network
        return (self.num_triangles() as f64) / (self.num_triplets() as f64);
    }

    pub fn friends_of_friends_prob(&self) -> f64 {
        // methid that calculates the average probability that friend of a user's friend
        // is also user's friend
        let mut sum_probs: f64 = 0.0;
        for user in 0..self.matrix.len() {
            let mut total_friends_friends: usize = 0;
            let mut friends_friends_friends: usize = 0;
            for friend in 0..self.matrix.len() {
                if self.matrix[user][friend] == 1 {
                    for friends_friend in 0..self.matrix.len() {
                        if (self.matrix[friend][friends_friend] == 1) && (friends_friend != user) {
                            total_friends_friends += 1;
                            if self.matrix[user][friends_friend] == 1 {
                                friends_friends_friends += 1;
                            }
                        }
                    }
                }
            }
            sum_probs += (friends_friends_friends as f64) / (total_friends_friends as f64);
        }
        sum_probs /= (self.num_vertices as f64);
        return sum_probs;
    }

    fn num_triangles_new(&self) -> usize {
        // method that returns the number of triangles
        return trace(&pow(&self.matrix, 3)) / 6;
    }

    fn num_triplets_new(&self) -> usize {
        // calculates and returns the number of triplets
        let mut ans: usize = 0;
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix.len() {
                ans += self.matrix[i][j];
            }
        }
        ans = (ans - trace(&self.matrix)) / 2;
        return ans;
    }
}

pub fn multiply(matrix1: &Vec<Vec<usize>>, matrix2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // multiplies two square matrices of the same size
    let mut answer: Vec<Vec<usize>> = vec![vec![0; matrix1.len()]; matrix1.len()];
    for col in 0..matrix1.len() {
        for row in 0..matrix1.len() {
            let mut value: usize = 0;
            for index in 0..matrix1.len() {
                value += matrix1[row][index] * matrix2[index][col];
            }
            answer[row][col] = value;
        }
    }
    return answer;
}

pub fn pow(matrix: &Vec<Vec<usize>>, power: usize) -> Vec<Vec<usize>> {
    // calculates and returns a power of a matrix
    //let mut answer: Vec<Vec<usize>> = vec![vec![0, matrix.len()], matrix.len()];
    let mut answer: Vec<Vec<usize>> = matrix.clone();
    for i in 1..power {
        answer = multiply(&answer, &matrix);
    }
    return answer;
}

pub fn trace(matrix: &Vec<Vec<usize>>) -> usize {
    // function that calculates and returns the trace of a square matrix
    let mut ans: usize = 0;
    for i in 0..matrix.len() {
        ans += matrix[i][i];
    }
    return ans;
}
