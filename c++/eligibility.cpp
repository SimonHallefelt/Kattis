#include <iostream>
using namespace std;

int main() {
    int n;
    cin >> n;

    for (int i = 0; i < n; i++) {
        string name;
        int studies;
        int birth;
        int courses;
        string trash;

        cin >> name;
        cin >> studies;
        cin >> trash;
        cin >> birth;
        cin >> trash;
        cin >> courses;

        cout << name << " ";
        if (studies >= 2010 || birth >= 1991)
        {
            cout << "eligible" << endl;
        }
        else if (courses > 40)
        {
            cout << "ineligible" << endl;
        }
        else
        {
            cout << "coach petitions" << endl;
        }
    }

    return 0;
}