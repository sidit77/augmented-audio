// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use daggy::NodeIndex;
use std::collections::HashMap;

#[derive(Default)]
pub struct DependencyGraph {
    graph: daggy::Dag<String, ()>,
    indexes: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn add_crate(&mut self, id: &str) {
        let idx = self.graph.add_node(id.into());
        self.indexes.insert(id.into(), idx);
    }

    pub fn has_crate(&self, id: &str) -> bool {
        self.indexes.contains_key(id)
    }

    pub fn add_dependency(&mut self, pkg: &str, dep: &str) {
        let idx1 = self.indexes[pkg];
        let idx2 = self.indexes[dep];
        self.graph.add_edge(idx1, idx2, ()).unwrap();
    }

    /// Sort dependencies for processing. Dependencies are ordered such that crates with no
    /// dependencies are listed first
    pub fn order_crates(&self) -> Vec<String> {
        let mut sorted_graph = daggy::petgraph::algo::toposort(&self.graph, None).unwrap();

        sorted_graph.reverse();
        sorted_graph
            .iter()
            .map(|idx| self.graph.node_weight(*idx).unwrap().to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order_crates() {
        let mut graph = DependencyGraph::default();
        graph.add_crate("crate-a");
        graph.add_crate("crate-b");
        graph.add_crate("crate-c");
        graph.add_crate("crate-d");

        graph.add_dependency("crate-a", "crate-b");
        graph.add_dependency("crate-a", "crate-c");
        graph.add_dependency("crate-b", "crate-c");
        graph.add_dependency("crate-c", "crate-d");

        let result = graph.order_crates();
        assert_eq!(result, vec!["crate-d", "crate-c", "crate-b", "crate-a"])
    }
}
