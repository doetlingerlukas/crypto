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

string shift_decode(int key, string input) {
    string alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    string to_return;
    for(char& c : input) {
        size_t found = alphabet.find(c);
        to_return += alphabet.at((found - key) % 26);
    }
    return to_return;
}

int main(int argc, char* argv[]) {

    if(argc == 2) {
        string input = (argv[1]);
        cout << "Input: " << input << endl;
        for(int i = 1; i <= 26; i++) {
            cout << "Encoded with key = " << i << ": " << shift_decode(i, input) << endl;
        }
    } else {
        cout << "Wrong arg count!" << endl;
    }
}