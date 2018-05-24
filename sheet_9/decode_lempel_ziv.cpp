#include <cstdlib> 
#include <cmath>  
#include <sstream>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

using namespace std;

vector<int> containedInPrev(string sequence, string to_check, int i) {
    int len = 0;
    bool b = false;

    while (!b) {
        string current = to_check.substr(0, len);
        cout << current << endl;
        size_t found = sequence.rfind(current, i-1);
        if (found == string::npos) {
            b = true;
        } else {
            len++;
        }
    }
    return {0, len};
}

vector<vector<int>> decodeNumber(vector<int> sequence) {
    int i = 0;

    vector<vector<int>> to_return;

    while (i < sequence.size()) {
        if (i == 0) {
            to_return.push_back({0, 0, sequence[0]});
            i++;
        } else {
            stringstream seq;
            stringstream to_check;
		    copy(sequence.begin(), sequence.end(), ostream_iterator<int>(seq, ""));
            copy(sequence.begin()+i, sequence.end(), ostream_iterator<int>(to_check, ""));
		    string s_bin = seq.str();
            string to_check_bin = to_check.str();
            vector<int> res = containedInPrev(s_bin, to_check_bin, i+1);
            int len = res[1];
            to_return.push_back({res[0]-i+1, len, sequence[i+len+1]});
            //cout << "(" << res[0]-i+1 << "," << len << "," << sequence[i+len+1] << ")" << endl;
            i += len;
        }
        
    }
    return to_return;
}

int main(int argc, char *argv[]) {
    
    if(argc == 2) {
        string input = (argv[1]);
        stringstream iss(input);
        
        char number;
        vector<int> sequence;
        while (iss.get(number)) {
			const char *to_convert = &number;
			sequence.push_back(atoi(to_convert));
		}
		
		for (vector<int> n : decodeNumber(sequence)) {
			cout << "(" << n[0] << "," << n[1] << "," << n[2] << ")" << endl;
		}
        
    } else {
		cout << "Wrong arg count!" << endl;
	}
}
