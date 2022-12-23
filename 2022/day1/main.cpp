#include <fstream>
#include <iostream>
#include <vector>

int main() {
  std::cout << "hello" << std::endl;
  std::ifstream input("input.txt");

  if (!input) {
    std::cout << "wrong input";
    return -1;
  }

  std::string line;
  int i = 0, num;
  std::vector<int> elves;

  while (getline(input, line)) {
    if (line == "") {
      i++;
      continue;
    }

    int calories = std::stoi(line);

    if (elves.size() == i) {
      elves.push_back(calories);
    } else {
      elves[i] += calories;
    }
  }

  int max = elves[0];
  int maxIndex = 0;

  for (i = 1; i < elves.size(); i++) {
    if (elves[i] > max) {
      max = elves[i];
      maxIndex = i;
    };
  }

  std::cout << "Max: " << max << std::endl;
  std::cout << "Elf: " << maxIndex << std::endl;

  std::sort(elves.rbegin(), elves.rend());

  std::cout << "Total of top 3: " << elves[0] + elves[1] + elves[2]
            << std::endl;
}