#include <iostream>
#include <fstream>
#include <sstream>
using namespace std;

int summenize(string Input, uint32_t skip) {
	int totall = 0;

	for (uint32_t i = 0; i < (Input).length(); i++) {
		if (Input[i] == Input[(i + skip) % Input.length()]) {
			totall += (Input[i] - '0');
		}
	}
	return totall;
}

string readfile() {
	ifstream f;
	f.open("Input.txt");
	stringstream buffer;

	buffer << f.rdbuf();
	string str = buffer.str();
	return str;
}

const string PUZZLE = readfile();
int main() {
	
	cout << summenize(PUZZLE, 1) << endl;
	cout << summenize(PUZZLE, PUZZLE.length() / 2) << endl;
}