# Unit conversion problem
Given a set of conversion facts which describe how one unit is converted to another, write a program to answer questions such as:
- How many m are in 10 ft?

Facts are given as `(unit1: string, amount: float, unit2: string)` tuples where the fact states how many of unit2 are in unit1.

Queries are given as `(value: float, from: string, to: string)` tuples. For example:
- `(10, "m", "ft")` represents: How many ft in 10 metres? or 10 metres in feet.

# Solution
This solution is to first build a graph, or multiple disjoint graphs, where each node has a unit and a HashMap of the neighbouring node keys (neighbouring units) mapped to the function by which a value of the unit could be converted to the given neighbouring unit.

This graph is represented in the form of a Map where unit string values are mapped to references of their corresponding GraphNode objects. This serves as a convenient way to start the conversion from one unit in the graph to another.

Once the graph is built, it can be searched (in this case a depth-first search, maybe there are better search algorithms for this problem) starting from and searching for the given units. Once a path is found, it can be traversed applying each of the conversion functions, from one unit to another, along the path. If there is no path found, the graph either does not contain the goal node or, the start node and the goal node exist in two separate disjoint graphs and there is no path between them.
