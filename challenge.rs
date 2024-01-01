use std::collections::*;
use std::hash::{Hash, Hasher};

type Fact = (&'static str, f32, &'static str);
type Query = (f32, &'static str, &'static str);

fn main() {
    let facts: Vec<Fact> = vec![
        ("m", 3.28, "ft"),
        ("ft", 12.0, "in"),
        ("in", 0.0277778, "yard")
        //("hr", 60.0, "min"),
        //("min", 60.0, "sec"),
    ];

    let queries: Vec<Query> = vec![(2.0, "m", "in"), (13.0, "in", "m"), (13.0, "in", "ft")];

    let unit_graph = build_unit_graph(&facts);
    for query in &queries {
        if let Some(conversion) = unit_graph.query(query) {
            println!("{0}{1} in {2} = {conversion}", query.0, query.1, query.2);
        } else {
            println!("No way to conver from {0} to {1}", query.1, query.2);
        }
    }
}

struct Graph {
    graph: HashMap<&'static str, GraphNode>,
}

impl Graph {
    fn dfs(&self, start_node_key: &'static str, search_node_key: &'static str) -> Option<VecDeque<&GraphNode>> {
        let mut stack = Vec::<&GraphNode>::new();
        let mut path = VecDeque::<&GraphNode>::new();
        let mut parent_keys = HashMap::<&str, &str>::new();
        let mut visited = HashSet::<&GraphNode>::new();

        // add the node where the search started
        stack.push(&self.graph[start_node_key]);

        while !stack.is_empty() {
            let mut head = stack.pop().unwrap();

            if head.unit == search_node_key {
                path.push_front(head);

                while let Some(parent_key) = parent_keys.get(head.unit) {
                    let parent = &self.graph[parent_key];
                    path.push_front(parent);
                    head = parent;
                }

                return Some(path);
            }
            visited.insert(head);

            for neighbour_key in head.neighbour_node_keys.keys() {
                let neighbour = &self.graph[neighbour_key];
                if visited.contains(neighbour) {
                    continue;
                }
                stack.push(neighbour);
                parent_keys.insert(neighbour_key, head.unit);
            }
        }

        return None;
    }

    fn query(&self, query: &Query) -> Option<f32> {
        let path = self.dfs(query.1, query.2)?;
        let mut path_iter = path.iter().peekable();
        let mut start_value = query.0;

        while let Some(node) = path_iter.next() {
            if let Some(next_node) = path_iter.peek() {
                let conversion_func = &node.neighbour_node_keys[next_node.unit];
                start_value = conversion_func(start_value);
            }
        }

        Some(start_value)
    }
}

struct GraphNode {
    unit: &'static str,
    neighbour_node_keys: HashMap<&'static str, Box<dyn Fn(f32) -> f32>>
}

impl Eq for GraphNode {
}

impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit
    }
}

impl Hash for GraphNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unit.hash(state);
        self.neighbour_node_keys.keys().collect::<Vec<_>>().hash(state);
    }
}

fn build_unit_graph(facts: &Vec<Fact>) -> Graph {
    let mut nodes = HashMap::<&str, GraphNode>::new();

    for fact in facts {
        let from_unit = &fact.0;
        let factor = fact.1;
        let to_unit = &fact.2;

        let from_node = nodes.entry(from_unit).or_insert(GraphNode {
            unit: from_unit,
            neighbour_node_keys: HashMap::new()
        });
        from_node.neighbour_node_keys.insert(to_unit, Box::new(move |value| value * factor));

        let to_node = nodes.entry(to_unit).or_insert(GraphNode {
            unit: to_unit,
            neighbour_node_keys: HashMap::new()
        });
        to_node.neighbour_node_keys.insert(from_unit, Box::new(move |value| value / factor));
    }

    Graph { graph: nodes }
}
