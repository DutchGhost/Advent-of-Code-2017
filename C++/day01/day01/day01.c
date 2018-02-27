#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
using namespace std;

const int SUBVAL = '0';

uint32_t summenize(char * Input, uint32_t skip) {
	uint32_t totall = 0;
	uint32_t LEN = strlen(Input) - 1;

	for (uint32_t i = 0; i < LEN; i++) {
		totall += (Input[i] - SUBVAL) & -(Input[i] == Input[i + skip]);
	}
	totall += (Input[0] - SUBVAL) & -(Input[0] == Input[LEN]);

	return totall;
}
/*
uint32_t optimized(string Input, uint32_t skip) {
	uint32_t totall = 0;
	uint32_t LEN = Input.length();

	for (uint32_t i = 0; i < LEN >> 1; i++) {
		totall += (Input[i] - SUBVAL) & -(Input[i] == Input[i + skip]);
	}
	return totall << 1;
}
*/

uint32_t optimized(char * Input, uint32_t skip) {
	uint32_t totall = 0;
	uint32_t LEN = strlen(Input) >> 1;

	for (uint32_t i = 0; i < LEN; i+=15) {
		totall += ((Input[i] - SUBVAL) & -(Input[i] == Input[i + skip])) +
			((Input[i + 1] - SUBVAL) & -(Input[i + 1] == Input[i + 1 + skip])) +
			((Input[i + 2] - SUBVAL) & -(Input[i + 2] == Input[i + 2 + skip])) +
			((Input[i + 3] - SUBVAL) & -(Input[i + 3] == Input[i + 3 + skip])) +
			((Input[i + 4] - SUBVAL) & -(Input[i + 4] == Input[i + 4 + skip])) +
			((Input[i + 5] - SUBVAL) & -(Input[i + 5] == Input[i + 5 + skip])) +
			((Input[i + 6] - SUBVAL) & -(Input[i + 6] == Input[i + 6 + skip])) +
			((Input[i + 7] - SUBVAL) & -(Input[i + 7] == Input[i + 7 + skip])) +
			((Input[i + 8] - SUBVAL) & -(Input[i + 8] == Input[i + 8 + skip])) +
			((Input[i + 9] - SUBVAL) & -(Input[i + 9] == Input[i + 9 + skip])) +
			((Input[i + 10] - SUBVAL) & -(Input[i + 10] == Input[i + 10 + skip])) +
			((Input[i + 11] - SUBVAL) & -(Input[i + 11] == Input[i + 11 + skip])) +
			((Input[i + 12] - SUBVAL) & -(Input[i + 12] == Input[i + 12 + skip])) +
			((Input[i + 13] - SUBVAL) & -(Input[i + 13] == Input[i + 13 + skip])) +
			((Input[i + 14] - SUBVAL) & -(Input[i + 14] == Input[i + 14 + skip]));
	}
	return totall << 1;
}


void readfile(char s[2190]) {
	FILE * pFile;

	fopen_s(&pFile, "Input.txt", "r");
	if (pFile == NULL) perror("Error opening file");
	else {
		if (fgets(s, 2191, pFile) != NULL)

		fclose(pFile);
	}
}

int main() {
	char PUZZLE[2190];
	readfile(PUZZLE);
	//cout << summenize(PUZZLE, 1) << endl;
	//cout << optimized(PUZZLE, strlen(PUZZLE) / 2) << endl;
	
	int end = 2190; //there are 2190 chars in the file
	int half = 2190 >> 1; //needed for 2nd part
	int start = 1; //start at 1,
	int prv = 0; //'cuz this is 0.
	int part1 = (PUZZLE[0] - SUBVAL) & -(PUZZLE[0] == PUZZLE[2189]); //look at the last and first char of the array, and do magic
	int part2 = 0; //part 2 is initialized at 0

	//loop by steps of 5, and update part 1 and 2 simulatniously.
	for (; half != end; prv += 5, half += 5, start += 5) {
		part1 += (PUZZLE[prv] - SUBVAL) & -(PUZZLE[prv] == PUZZLE[start]);
		part2 += (PUZZLE[prv] - SUBVAL) & -(PUZZLE[prv] == PUZZLE[half]);

		part1 += (PUZZLE[prv + 1] - SUBVAL) & -(PUZZLE[prv + 1] == PUZZLE[start + 1]);
		part2 += (PUZZLE[prv + 1] - SUBVAL) & -(PUZZLE[prv + 1] == PUZZLE[half + 1]);

		part1 += (PUZZLE[prv + 2] - SUBVAL) & -(PUZZLE[prv + 2] == PUZZLE[start + 2]);
		part2 += (PUZZLE[prv + 2] - SUBVAL) & -(PUZZLE[prv + 2] == PUZZLE[half + 2]);

		part1 += (PUZZLE[prv + 3] - SUBVAL) & -(PUZZLE[prv + 3] == PUZZLE[start + 3]);
		part2 += (PUZZLE[prv + 3] - SUBVAL) & -(PUZZLE[prv + 3] == PUZZLE[half + 3]);

		part1 += (PUZZLE[prv + 4] - SUBVAL) & -(PUZZLE[prv + 4] == PUZZLE[start + 4]);
		part2 += (PUZZLE[prv + 4] - SUBVAL) & -(PUZZLE[prv + 4] == PUZZLE[half + 4]);
	}

	//fix part 1.
	for (; start != end; prv += 2, start += 2) {
		part1 += (PUZZLE[prv] - SUBVAL) & -(PUZZLE[prv] == PUZZLE[start]);
		part1 += (PUZZLE[prv + 1] - SUBVAL) & -(PUZZLE[prv + 1] == PUZZLE[start + 1]);
	}

	cout << part1 << endl;
	cout << part2 * 2 << endl;
}