#include <vector>
#include <unordered_map>
#include <queue>
using namespace std;

vector<string> BFS(unordered_map<string, vector<string>> map, string startNode, string endNode) {
    if (startNode == endNode) return {"startNode"};
    unordered_map<string, string> previous = {{startNode, startNode}};
    queue<string> q; q.push(startNode);
    vector<string> path = {};
    
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

