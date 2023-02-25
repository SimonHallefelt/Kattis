#include <iostream>
#include <string>
#include <vector>

using namespace std;
typedef long long ll;
const int INF = 1e9;
const ll MOD = (1e9)+7;

vector<string> split(string s) {
    vector<string> v = vector<string>();
    string temp = "";
    for (int i = 0; i < s.size(); i++)
    {
        if (s[i] == ' ')
        {
            v.push_back(temp);
            temp = "";
        }else
        {
            temp += s[i];
        }        
    }
    v.push_back(temp);
    return v;
}

bool isTrue(vector<string> v, vector<bool> variebels) {
    bool oneTrue = false;
    for (int i = 0; i < v.size(); i += 2)
    {
        if (v[i][0] == '~'){
            if (!variebels[stoi(v[i].substr(2, v[i].size())) - 1])
            {
                return true;
            }
        }else{
            if (variebels[stoi(v[i].substr(1, v[i].size())) - 1])
            {
                return true;
            }
        }
    }
    
    return oneTrue;
}

bool testAll(vector<vector<string>> v, vector<bool> variebels, int count) {
    bool allTrue = true;
    for (int i = 0; i < v.size(); i++)
    {
        if (!isTrue(v[i], variebels))
        {
            allTrue = false;
            break;
        }
    }
    if (allTrue)
    {
        return true;
    }
    for (int i = count; i < variebels.size(); i++)
    {
        variebels[i] = false;
        if(testAll(v, variebels, i + 1)){
            return true;
        }
        variebels[i] = true;
    }
    return false;
}


int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int times;
    cin >> times;

    for (int i = 0; i < times; i++)
    {
        int n, m;
        cin >> n >> m;
        string trash;
        getline(cin, trash);

        vector<vector<string>> v = vector<vector<string>>(m);
        for (int i = 0; i < m; i++)
        {
            string s;
            vector<string> v2 = vector<string>();
            getline(cin, s);
            v2 = split(s);
            v[i] = v2;
        }

        vector<bool> variebels = vector<bool>(n);
        for (int i = 0; i < n; i++)
        {
            variebels[i] = true;
        }

        if (testAll(v, variebels, 0))
        {
            cout << "satisfiable" << endl;
        }else{
            cout << "unsatisfiable" << endl;
        }
    }
    return 0;
}

// g++ file.cpp (g++ file.cpp -o file)