#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <queue>
#include <algorithm>
using namespace std;

unordered_map<string, int> distances;
vector<string> BFS(unordered_map<string, vector<string>> map, string startNode, string endNode) {
    if (startNode == endNode) return {startNode};
    unordered_map<string, string> previous = {{startNode, startNode}};
    queue<string> q; q.push(startNode);
    vector<string> path = {};
    distances[startNode] = 0;
    
    while (q.size()) {
        auto node = q.front(); q.pop();
        for (auto neighbor : map[node]) {
            if (previous.find(neighbor) == previous.end()) {
                previous[neighbor] = node;
                q.push(neighbor);
                distances[neighbor] = distances[node]+1;
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

int main() {
    vector<string> input = {};
    while (1) {
        string inputRow;
        getline(cin, inputRow);
        // cin >> inputRow;
        if (inputRow.length() == 0) {
            break;
        }
        input.push_back(inputRow);
    }

    vector<string> firstNames = {};
    unordered_map<string, vector<string>> map;

    string del = " ";
    for (size_t i = 0; i < input.size(); i++) {
        string row = input[i];
        auto pos = row.find(del);
        string firstName = row.substr(0, pos);
        row = row.substr(pos+1, row.size()-pos);
        pos = row.find(del);

        firstNames.push_back(firstName);
        if (map.find(firstName) == map.end()) {
            map[firstName] = {};
        }
        string name;
        while (pos != string::npos) {
            name = row.substr(0, pos);
            row = row.substr(pos+1, row.size()-pos);
            pos = row.find(del);

            if (map.find(name) == map.end()) {
                map[name] = {};
            }
            map[name].push_back(firstName);
            map[firstName].push_back(name);
        }
        if (map.find(row) == map.end()) {
            map[row] = {};
        }
        map[row].push_back(firstName);
        map[firstName].push_back(row);
    }


    string target = "PAUL_ERDOS";
    vector<string> path = BFS(map, target, "bla");
    for (auto name : firstNames) {
        if (name == target) {
            cout << name << " 0" << endl;
        } else if (distances[name]) {
            cout << name << " " << distances[name] << endl;
        } else {
            cout << name << " no-connection" << endl;
        }
    }
    
}