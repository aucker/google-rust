# include<iostream>
# include<bits/stdc++.h>
# include<vector>
using namespace std;
int main() {
    int T;
    cin >> T;
    int i = 1;
    while (i <= T) {
        int N;
        cin >> N;
        vector<int> vec;
        for (int i = 1; i <= N; i++) {
            int a;
            cin >> a;
            vec.push_back(a);
        }
        sort(vec.begin(), vec.end());
        int legal = 0;
        int k = 1;
        for (int j = 0; j < N; j++) {
            if (k < vec[j]) {
                k++;
                legal++;
            }
        }

        cout << "Case #" << i << ": " << N - legal << endl;
        ++i;
    }
}
// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a46471