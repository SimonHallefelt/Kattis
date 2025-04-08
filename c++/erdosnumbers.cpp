#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <queue>
#include <algorithm>
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
    for (size_t i = 0; i < firstNames.size(); i++) {
        vector<string> path = BFS(map, firstNames[i], target);
        if (path.size()) {
            cout << firstNames[i] << " " << path.size()-1 << endl;
        } else {
            cout << firstNames[i] << " no-connection" << endl;
        }
    }
    
}