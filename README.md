# Rust-Genetic

In this work, I suggest considering the implementation of a prototype genetic algorithm for the traveling salesman problem using the Rust programming language.

The diploma thesis proposes the implementation of a flexible, parameterized genetic algorithm for finding optimal solutions to the traveling salesman problem, as well as a subprogram for visualizing the process of finding such solutions.

The developed algorithm can be used to investigate and solve other NP-complex problems with a given accuracy. Each probability of genetic transition is parameterized and can be conveniently tuned to a specific problem.

I developed a system of step-by-step visualization for each population generation and a visualization of the fitness function of each generation. This makes it easy to visually control the evolution of each generation of the genetic algorithm iteration and select parameters for a more accurate solution of the problem.

# To run

```
cargo run ./config0.csv ./cities0.csv > ./output0.csv
```

# To plot
```
python3 plot.py ./cities0.csv ./output0.csv
```

![](pict/Video.gif)

# License

MIT
