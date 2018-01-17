#include <iostream>
#include <fstream>
#include <sstream>
using namespace std;

const int SUBVAL = '0';

uint32_t summenize(string Input, uint32_t skip) {
	uint32_t totall = 0;
	uint32_t LEN = Input.length() - 1;

	for (uint32_t i = 0; i < LEN; i++) {
		if (Input[i] == Input[i + skip]) {
			totall += (Input[i] - SUBVAL);
		}
	}

	if (Input[0] == Input[LEN]) {
		totall += Input[0] - SUBVAL;
	}
	return totall;
}

uint32_t optimized(string Input, uint32_t skip) {
	uint32_t totall = 0;
	uint32_t LEN = Input.length();

	for (uint32_t i = 0; i < LEN >> 1; i++) {
		if (Input[i] == Input[i + skip]) {
			totall += (Input[i] - SUBVAL);
		}
	}
	return totall << 1;
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
	cout << optimized(PUZZLE, PUZZLE.length() / 2) << endl;
}