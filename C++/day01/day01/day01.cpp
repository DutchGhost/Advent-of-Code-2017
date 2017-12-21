#include <iostream>
#include <fstream>
#include <sstream>
using namespace std;

uint32_t summenize(string Input, uint32_t skip) {
	int totall = 0;

	for (uint32_t i = 0; i < Input.length(); i++) {
		if (Input[i] == Input[(i + skip) % Input.length()]) {
			totall += (Input[i] - '0');
		}
	}
	return totall;
}

uint32_t optimized(string Input, uint32_t skip) {
	int totall = 0;

	for (uint32_t i = 0; i < Input.length() >> 1; i++) {
		if (Input[i] == Input[(i + skip) % Input.length()]) {
			totall += (Input[i] - '0');
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