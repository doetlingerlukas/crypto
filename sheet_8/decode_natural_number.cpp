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


vector<int> decodeNumber(vector<int> sequence) {
	// standard length is 3
	int len = 3;
	int i = 0;
	
	vector<int> to_return;
	
	while (i < sequence.size()) {
		if (i+len > sequence.size()) {
			throw invalid_argument("Invalid sequence!");
		}
		stringstream sub_seq;
		copy(sequence.begin() + i, sequence.begin() + i + len, ostream_iterator<int>(sub_seq, "")); 
		string s_bin = sub_seq.str();
		int n = stoi(s_bin, nullptr, 2);
		i += len;
		
		if (sequence[i] == 0) {
			i++;
			to_return.push_back(n);
			len = 3;
		} else {
			len = n;
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
		
		for (int n : decodeNumber(sequence)) {
			cout << n << endl;
		}
        
    } else {
		cout << "Wrong arg count!" << endl;
	}
}
