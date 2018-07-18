#Day 26: Karger’s mincut
<br>
Given a discrete graph with set of vertices V and set of edges G, mincut is a split of V into two disjoint non-empty sets V1 and V2, such that set {(u,v)|(u,v)∈G, u∈V1, v∈V2} has minimal size. Or a friendly explanation, remove the least amount of edges possible so that the graph falls apart.
<br>
Karger’s mincut is a probabilistic algorithm that uses edge contraction with uniform selection. When an edge is contracted, remaining edges must be preserved to increase a chance of new multi-edges to be chosen on the next contraction.
<br>
When the algorithm is called N² times on a graph with N vertices, the probability that mincut is found is approximately 1–1/N.
<br>
generate graph:

```
let graph = vec![
   (String::from("A1"), String::from("A2")),
   (String::from("A1"), String::from("A3")),
   (String::from("A1"), String::from("A4")),
   (String::from("A2"), String::from("A3")),
   (String::from("A2"), String::from("A5")),
   (String::from("A3"), String::from("A4")),
   (String::from("A4"), String::from("A5")),
];
```

<br>

run:

```
best cut::2
component#1:A1,A3,A4,A2
component#2:A5
```
