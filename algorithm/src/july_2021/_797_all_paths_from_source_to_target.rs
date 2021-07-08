struct Solution;

impl Solution {
    fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(n: i32, length: usize, graph: &[Vec<i32>], path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
            path.push(n);
            if n as usize == length - 1 {
                paths.push(path.clone());
            } else {
                for &v in &graph[n as usize] {
                    dfs(v, length, graph, path, paths);
                }
            }
            path.pop();
        }
        let mut res = vec![];
        let mut path = vec![];
        dfs(0, graph.len(), &graph, &mut path, &mut res);
        res
    }
}