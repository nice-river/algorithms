#include <bits/stdc++.h>
using namespace std;

int main() {
    int n;
    char c;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> c;
        if (c == 'B' || c == 'b') {
            cout << "BattleShip\n";
        } else if (c == 'C' || c == 'c') {
            cout << "Cruiser\n";
        } else if (c == 'D' || c == 'd') {
            cout << "Destroyer\n";
        } else if (c == 'F' || c == 'f') {
            cout << "Frigate\n";
        }
    }
    return 0;
}