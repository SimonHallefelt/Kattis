#include <iostream>

using namespace std;
typedef long long ll;
const int INF = 1e9;
const ll MOD = (1e9)+7;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    long n; cin >> n;
    long differences[n];
    for (size_t i = 0; i < n; i++)
    {
        cin >> differences[i];
    }

    long initial = 0;
    long temp2 = 0;
    for (size_t i = 0; i < n; i++)
    {
        temp2 = temp2 - differences[i];
        initial += temp2;
    }

    long b1 = 0;
    long temp = initial / (n+1);
    if (initial == 0)
    {
        b1 = 0;
    }else if (initial > 0)
    {
        if(initial - (n+1)*temp > (initial - (n+1)*(temp+1))*-1){
            b1 -= temp+1;
        }else{
            b1 -= temp;
        }
    }else
    {
        if((initial - (n+1)*temp)*-1 < initial - (n+1)*(temp-1)){
            b1 -= temp;
        }else{
            b1 -= temp-1;
        }
    }

    for (size_t i = 0; i < n; i++)
    {
        cout << b1 << " ";
        b1 -= differences[i];
    }
    cout << b1;
}

// g++ file.cpp (g++ file.cpp -o file)