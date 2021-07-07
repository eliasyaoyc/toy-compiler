//! 96
struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // class Solution {
        //     private int[][] memo;
        //
        //     public int numTrees(int n) {
        //     memo = new int[n + 1][n + 1];
        //     return count(1,n);
        //     }
        //
        //     private int count(int lo, int hi) {
        //     if (lo > hi) return 1;
        //     if (memo[lo][hi] != 0) {
        //     return memo[lo][hi];
        //     }
        //
        //     int res = 0;
        //     for(int mid = lo; mid <= hi; mid++){
        //     int left = count(lo, mid - 1);
        //     int right = count(mid + 1, hi);
        //     res += left * right;
        //     }
        //     memo[lo][hi] = res;
        //     return res;
        //     }
        // }
        1
    }
}

#[test]
fn t() {
    for i in 1..3 {
        println!("{}", i);
    }
}