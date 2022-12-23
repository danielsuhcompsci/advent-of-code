#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int main() {
  std::ifstream input("input.txt");
  std::string line;

  std::unordered_map<char, char> map = {{'A', 'X'}, {'B', 'Y'}, {'C', 'Z'}};
  std::unordered_map<char, char> winMap = {{'A', 'C'}, {'B', 'A'}, {'C', 'B'}};
  std::unordered_map<char, int> scoreMap = {{'X', 1}, {'Y', 2}, {'Z', 3}};
  std::unordered_map<char, std::string> debugMap = {
      {'X', "Rock"}, {'Y', "Paper"}, {'Z', "Scissors"}};

  int score = 0;
  int i = 0;
  int lineCount = 0;

  // 'X' is rock(1), 'Y' is paper(2), 'Z' is scissors(3)
  while (getline(input, line)) {
    lineCount++;
    char opponent = line[0];
    char me = line[2];

    // int scorePrev = score;

    if (winMap[me] == opponent)  // win
      score += 6 + scoreMap[me];
    else if (me == opponent)  // draw
      score += 3 + scoreMap[me];
    else  // loss
      score += scoreMap[me];

    // if (i < 3) {
    //   std::cout << "Round " << i << ": " << debugMap[opponent] << " vs "
    //             << debugMap[me] << " Score: " << score - scorePrev <<
    //             std::endl;
    //   i++;
    // }
  }

  std::cout << lineCount << std::endl;
  std::cout << score << std::endl;
}