use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn from_file(path: &str) -> Self {
        let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
        let file = std::fs::read_to_string(path).unwrap();
        for line in file.lines() {
            if line.is_empty() {
                continue;
            }

            let (node, neighbor) = line.split_once(" -> ").expect("bad graph format");

            nodes
                .entry(node.to_string())
                .or_default()
                .push(neighbor.to_string());
        }
        Self { nodes }
    }

    pub fn hamiltonian_paths(&self) -> Vec<Vec<String>> {
        let mut paths = vec![];
        for node in self.nodes.keys() {
            let mut path = vec![];
            self._hamiltonian_paths(node, &mut path, &mut paths);
        }
        paths
    }

    fn _hamiltonian_paths(&self, node: &str, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
        path.push(node.to_string());
        if path.len() == self.nodes.len() {
            paths.push(path.clone());
        } else {
            for neighbor in &self.nodes[node] {
                if !path.contains(neighbor) {
                    self._hamiltonian_paths(neighbor, path, paths);
                }
            }
        }
        path.pop();
    }
}
