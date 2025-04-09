#include <vector>
#include <unordered_map>
#include <queue>
#include <algorithm>
using namespace std;

vector<int> BFS(unordered_map<int, vector<int>>& map, int startNode, int endNode) {
    if (startNode == endNode) return {startNode};
    unordered_map<int, int> previous = {{startNode, startNode}};
    queue<int> q; q.push(startNode);
    vector<int> path = {};
    
    while (q.size()) {
        auto node = q.front(); q.pop();
        for (auto neighbor : map[node]) {
            if (previous.find(neighbor) == previous.end()) {
                previous[neighbor] = node;
                q.push(neighbor);
                if (neighbor == endNode) {
                    path.push_back(neighbor);
                    while (neighbor != startNode) {
                        neighbor = previous[neighbor];
                        path.push_back(neighbor);
                    }
                    goto exit_loops;
                }
            }
        }
    }

    exit_loops:
    reverse(path.begin(), path.end());
    return path;
}


#include <cassert>
#include <iostream>
int main() {
    unordered_map<int, vector<int>> graph = {
        {1, {2,3}},
        {2, {3,4}},
        {3, {2,3,4,5}},
        {4, {2,3,4}},
        {5, {4,6}},
        {6, {5}},
        {7, {8}},
        {8, {7}}
    };
    assert(1 == BFS(graph, 1, 1).size());
    assert(4 == BFS(graph, 1, 6).size());
    assert(0 == BFS(graph, 1, 8).size());
    assert(2 == BFS(graph, 7, 8).size());
    assert(2 == BFS(graph, 8, 7).size());

    assert(0 != BFS(graph, 7, 7).size());
    cout << "Pass";
}