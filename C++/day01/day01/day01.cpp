#include <iostream>
#include <fstream>
#include <sstream>
using namespace std;

int summenize(string* str, uint32_t skip) {
	int totall = 0;

	for (uint32_t i = 0; i < (*str).length(); i++) {
		if ((*str)[i] == (*str)[(i + skip) % (*str).length()]) {
			totall += ((*str)[i] - '0');
		}
	}
	return totall;
}

int main() {
	ifstream f;
	f.open("Input.txt");
	stringstream buffer;

	buffer << f.rdbuf();
	string str = buffer.str();
	
	cout << summenize(&str, 1) << endl;
	cout << summenize(&str, str.length() / 2) << endl;
}