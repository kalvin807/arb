use std::collections::HashMap;

type FX = HashMap<String, HashMap<String, f32>>;

fn main() {
    let fx: FX = HashMap::from([
        (
            "a".to_string(),
            HashMap::from([("b".to_string(), -1.0), ("c".to_string(), 4.0)]),
        ),
        (
            "b".to_string(),
            HashMap::from([
                ("c".to_string(), 3.0),
                ("d".to_string(), 2.0),
                ("e".to_string(), 2.0),
            ]),
        ),
        ("c".to_string(), HashMap::from([])),
        (
            "d".to_string(),
            HashMap::from([("b".to_string().to_string(), 1.0), ("c".to_string(), 5.0)]),
        ),
        ("e".to_string(), HashMap::from([("d".to_string(), -3.0)])),
    ]);
    let (d, p) = bellman_ford(fx, "a".to_string());
    println!("Hello, world!");
    println!("{:#?}", d);
    println!("{:#?}", p);
}

fn bellman_ford<'a>(
    graph: HashMap<String, HashMap<String, f32>>,
    source: String,
) -> (HashMap<String, f32>, HashMap<String, String>) {
    let node_size = graph.len();
    let mut distance: HashMap<String, f32> = HashMap::with_capacity(node_size);
    let mut predecessor: HashMap<String, String> = HashMap::with_capacity(node_size);

    /*
     * Init graph
     */
    for (node, _) in &graph {
        distance.insert(node.clone(), f32::INFINITY);
        predecessor.insert(node.clone(), "".to_string());
    }
    // Source -> source dist is zero
    *distance.get_mut(&source).unwrap() = 0.0;

    /*
     * Relax edges, O(V3)
     */
    for _ in 0..node_size - 1 {
        for (node, node_neighbor) in &graph {
            for (neighbor, _) in node_neighbor {
                if distance[neighbor] > distance[node] + graph[node][neighbor] {
                    *distance.get_mut(neighbor).unwrap() = distance[node] + graph[node][neighbor];
                    *predecessor.get_mut(neighbor).unwrap() = (&node).to_string();
                }
            }
        }
    }

    /*
     * Check if negative cycle exist
     */
    for (node, neighbors) in &graph {
        for (neighbor, _) in neighbors {
            if distance[neighbor] <= distance[node] + graph[node][neighbor] {
                println!("negative weight cycle")
            }
        }
    }
    return (distance, predecessor);
}
