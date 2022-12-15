mod resources;

use crate::resources::network::networks_info;
use crate::resources::network::read_file;
use crate::resources::network::sort_by_cluestering_coefficient;
use crate::resources::network::sort_by_density;
use crate::resources::network::sort_by_friend_nums;
use crate::resources::network::sort_by_friends_friends_prob;
use crate::resources::network::Network;

fn main() {
    let graph_de: Network = read_file("musae_DE_edges.csv", "Germany");
    let graph_engb: Network = read_file("musae_ENGB_edges.csv", "Great Britain");
    let graph_es: Network = read_file("musae_ES_edges.csv", "Spain");
    let graph_fr: Network = read_file("musae_FR_edges.csv", "France");
    let graph_ptbr: Network = read_file("musae_PTBR_edges.csv", "Portugal and Brazil");
    let graph_ru: Network = read_file("musae_RU_edges.csv", "Russia");
    let vector_of_graphs: Vec<Network> = vec![
        graph_de, graph_engb, graph_es, graph_fr, graph_ptbr, graph_ru,
    ];

    //graphs_info(&vector_of_graphs);
    //matrices_info(&vector_of_graphs);
    networks_info(&vector_of_graphs);
    sort_by_friend_nums(&vector_of_graphs);
    sort_by_density(&vector_of_graphs);
    sort_by_cluestering_coefficient(&vector_of_graphs);
    sort_by_friends_friends_prob(&vector_of_graphs);

    // test for graph info and matrix info
    //let test: Network = read_file("test_data.csv", "test");
    //let vector_test: Vec<Network> = vec![test];
    //graphs_info(&vector_test);
    //matrices_info(&vector_test);
    //networks_info(&vector_test);

    /*
    tests for matrix multiplication
    let matrix1: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix2: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
    let result1: Matrix = Matrix::new_vector(&multiply(&matrix1, &matrix2));
    let result2: Matrix = Matrix::new_vector(&multiply(&matrix2, &matrix1));
    result1.print();
    result2.print();
    */

    /*
    test for matrix power
    let matrix_power: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result_power_1: Matrix = Matrix::new_vector(&pow(&matrix_power, 2));
    let result_power_2: Matrix = Matrix::new_vector(&pow(&matrix_power, 3));
    result_power_1.print();
    result_power_2.print();
    */
}
