#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

vector<int> bit;

int query(int idx) {            // sum of first idx elements, idx is a[i] 1-indexed position in b
    int res = 0;
    while (idx > 0) {           // idx is 1-indexed
        res += bit[idx];        // add the value at idx
        idx -= idx & -idx;      // move to the parent node
        cout << "idx: " << idx << endl;
    }
    return res;
}

void update(int idx, int val) {     // add val to idx, idx is a[i] 1-indexed position in b
    while (idx < bit.size()) {      // idx is 1-indexed and bit.size() is n + 1
        bit[idx] += val;            // add val to idx
        idx += idx & -idx;          // move to the next child node
    }
}

int main() {    // BIT
    int n;
    cin >> n;
    vector<int> a(n), b(n);
    bit.resize(n + 1);
    long total = 0;
    for (int i = 0; i < n; i++) {
        cin >> a[i];
        b[i] = a[i];
    }
    sort(b.begin(), b.end());                                               // sort b to get the 1-indexed position of each element in a
    for (int i = 0; i < n; i++) {
        int pos = lower_bound(b.begin(), b.end(), a[i]) - b.begin() + 1;    // 1-indexed position of a[i] in b
        int num_swaps = i - query(pos);                                     // number of elements to the left of pos that are greater than a[i]
        update(pos, 1);                                                     // mark a[i] as visited
        total += num_swaps;
    }
    cout << total << endl;
    return 0;
}
