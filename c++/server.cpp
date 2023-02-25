#include <iostream>

using namespace std;
typedef long long ll;
const int INF = 1e9;
const ll MOD = (1e9)+7;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n, time;
    cin >> n >> time;

    int i = 0;
    int sum = 0;
    while (i < n)
    {
        int t;
        cin >> t;
        sum += t;
        if (sum > time)
        {
            cout << i << endl;
            return 0;
        }
        i++;
    }
    cout << n << endl;
    return 0;
    
}