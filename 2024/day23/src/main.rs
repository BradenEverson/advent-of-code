use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("data/input").expect("Failed to find file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let connections: Vec<_> = buf
        .lines()
        .filter_map(|line| line.split_once('-'))
        .collect();

    let graph = LanGraph::from_connections(&connections);
    let interconnections = graph
        .find_triangles()
        .into_iter()
        .filter(|triangle| triangle.iter().filter(|node| node.starts_with('t')).count() > 0)
        .collect::<Vec<_>>();

    println!("{}", interconnections.len())
}

#[derive(Default, Debug)]
pub struct LanNode<'a> {
    connections: Vec<&'a str>,
}

impl<'a> LanNode<'a> {
    pub fn connect(&mut self, to: &'a str) {
        if !self.connections.contains(&to) {
            self.connections.push(to);
        }
    }
}

#[derive(Debug)]
pub struct LanGraph<'a> {
    nodes: HashMap<&'a str, LanNode<'a>>,
}

impl<'a> LanGraph<'a> {
    pub fn from_connections(connections: &[(&'a str, &'a str)]) -> Self {
        let mut nodes = HashMap::new();

        for (left, right) in connections {
            nodes.entry(*left).or_insert(LanNode::default());
            nodes.entry(*right).or_insert(LanNode::default());
        }

        for (left, right) in connections {
            nodes.get_mut(left).unwrap().connect(right);
            nodes.get_mut(right).unwrap().connect(left);
        }

        Self { nodes }
    }

    pub fn find_triangles(&self) -> Vec<Vec<&'a str>> {
        let mut triangles = Vec::new();

        for (&node, lan_node) in &self.nodes {
            let neighbors: Vec<&str> = lan_node.connections.iter().cloned().collect();

            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    let neighbor_a = neighbors[i];
                    let neighbor_b = neighbors[j];

                    if self.nodes[neighbor_a].connections.contains(&neighbor_b) {
                        let mut triangle = vec![node, neighbor_a, neighbor_b];
                        triangle.sort();
                        if !triangles.contains(&triangle) {
                            triangles.push(triangle);
                        }
                    }
                }
            }
        }
        triangles
    }
}
