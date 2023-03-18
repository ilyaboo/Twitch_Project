# Twitch Network Analysis
### Project Summary
In this project I managed to write a code that extracts information about the network of Twitch users from the data files, generates different representations of this network (list of connections in the graph and adjacency matrix), implemented a number of data analysis algorithms including calculations of number of edges and vertices, average number of friends of users in the network, number of triangles in the graph, number of triples in the graph, graph density, clustering coefficient, and average probability that user’s friend’s friend is also user’s friend. I also implemented functions for matrices such as matrix multiplication, power of a matrix, as well as calculation of number of triangles and number of triplets using properties of matrices. However, I did not use these algorithms due to their low efficiency with large matrices.
### Results Presentation
Below are the screenshots of the code output

![pic1](https://user-images.githubusercontent.com/108785038/226074889-ef543067-e046-4b65-a77d-03776ca33b3e.png)
![pic2](https://user-images.githubusercontent.com/108785038/226074891-f93585c6-02e5-40a9-9889-a1beb6c71c8f.png)
![pic3](https://user-images.githubusercontent.com/108785038/226074894-01a193a4-8174-44e3-aa08-37f50347865b.png)
### Results Analysis
I then compared the countries in terms of the calculated metrics and got the following.
![pic4](https://user-images.githubusercontent.com/108785038/226074906-e6c16441-8478-489e-a12f-94476a6b9bdf.png)
From this table we can see how all these metrics are closely related and representative in showing how closely connected users in each community are. For example, Portugal and Brazil region keeps its first place for 3 and gets a second in the fourth. Similar, but opposite performance shows Great Britain. Due to such consistency in results, there is enough evidence to claim that out of these 6 regions, Portugal and Brazil region has the most closely connected user networks, whereas Great Britain has the least closely connected one. It can also be noticed that even though France has the biggest average number of friends per user, according to all the other metrics France gets the third place. That yields that friend connection in France are uniformly distributed among the graph, whereas connections in Portugal and Brazil form community clusters. Other noticeable fact is that with a relatively small average number of friends in Spain network, it has a second place in all other metrics, meaning that these connections form community clusters inside the network.
To conclude, such a complacent table gives an insight about qualitative differences between twitch user networks in different countries.
