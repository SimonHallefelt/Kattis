#include <iostream>
#include <map>
#include <sstream>

using namespace std;
typedef long long ll;
const int INF = 1e9;
const ll MOD = (1e9)+7;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    string ALPABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    std::map<std::string, int> AlpabetMap;
    for (size_t i = 0; i < ALPABET.size(); i++)
    {
        AlpabetMap[ALPABET.substr(i, 1)] = i;
    }

    int n; cin >> n;
    for (size_t i = 0; i < n; i++)
    {
        int posShift; cin >> posShift;
        string trash; getline(cin, trash);
        string s; getline(cin, s);
        for (size_t j = 0; j < s.size(); j++)
        {
            if(AlpabetMap.count(s.substr(j, 1)) == 0){
                cout << s.substr(j, 1);
            }else{
                int pos = AlpabetMap[s.substr(j, 1)];
                pos = (pos - posShift + 26)  % 26;
                cout << ALPABET[pos];
            }
        }
        cout << endl;
    }
}

// g++ file.cpp (g++ file.cpp -o file)