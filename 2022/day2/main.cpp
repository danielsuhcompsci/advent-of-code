#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

int main() {
  std::ifstream input("input.txt");
  std::string line;

  std::unordered_map<char, char> charToInt = {{'A', 0}, {'B', 1}, {'C', 2},
                                              {'X', 0}, {'Y', 1}, {'Z', 2}};
  std::unordered_map<int, std::string> debugMap = {
      {0, "Rock"}, {1, "Paper"}, {2, "Scissors"}};

  int score1 = 0;
  int score2 = 0;
  int i = 0;

  // 'X' is rock(1), 'Y' is paper(2), 'Z' is scissors(3)
  while (getline(input, line)) {
    int opponent = charToInt[line[0]];
    int me = charToInt[line[2]];

    // int scorePrev = score;

    // First part
    switch ((me - opponent + 3) % 3) {
      case 0:  // draw
        score1 += 3 + me + 1;
        break;
      case 1:  // win
        score1 += 6 + me + 1;
        break;
      case 2:  // lose
        score1 += me + 1;
        break;
      default:
        std::cout << "Invalid input: " << ((me - opponent + 3) % 3)
                  << std::endl;
        return -1;
    }

    // Second part
    switch (me) {
      case 0:  // need to lose
        score2 += ((opponent - 1 + 3) % 3) + 1;
        break;
      case 1:  // need to draw
        score2 += 3 + opponent + 1;
        break;
      case 2:  // need to win
        score2 += 6 + ((opponent + 1) % 3) + 1;
        break;
      default:
        std::cout << "Invalid input: " << me << std::endl;
        return -1;
    }

    // if (i < 3) {
    //   std::cout << "Round " << i << ": " << debugMap[opponent] << " vs "
    //             << debugMap[me] << " Score: " << score - scorePrev <<
    //             std::endl;
    //   i++;
    // }
  }

  std::cout << score1 << std::endl;
  std::cout << score2 << std::endl;
}