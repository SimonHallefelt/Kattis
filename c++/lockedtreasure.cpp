#include <iostream>

using namespace std;
typedef long long ll;
const int INF = 1e9;
const ll MOD = (1e9)+7;
    
int binomial(int n, int k) {
    int C[n + 1][k + 1]; //det fungerar din jävel
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= min(i, k); j++) {
            if (j == 0 || j == i) {
                C[i][j] = 1;
            } else {
                C[i][j] = C[i-1][j-1] + C[i-1][j];
            }
        }
    }
    return C[n][k];
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int times;
    cin >> times;

    int i = 0;
    while (i < times)
    {
        int n, m;
        cin >> n >> m;

        if (m == 1)
        {
            cout << "1" << endl;
        }else
        {
            cout << binomial(n, m-1) << endl;
        }
        
        
        i++;
    }
    

    return 0;
}

// 1,2
// 1,3
// 1,4
// 1,5
// 2,3

// 2,4
// 2,5
// 3,4
// 3,5
// 4,5


// g++ file.cpp (g++ file.cpp -o file)