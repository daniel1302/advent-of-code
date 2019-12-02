#include<iostream>
#include<vector>
#include<string>
#include<fstream>
#include<numeric>

std::vector<int> readInput(std::string fileName)
{
    std::ifstream input{fileName};
    std::vector<int> output{};
    std::string line;
    while(getline(input, line)) {
        output.push_back(std::stoi(line));
    }

    return output;
}

int part1 (const std::vector<int>& input) {
    return std::accumulate(input.begin(), input.end(), 0, [](int acc, int v) {
        return acc + (v / 3) - 2;
    });
}

int part2 (const std::vector<int>& input) {
    return std::accumulate(input.begin(), input.end(), 0, [](int acc, int v) {
        int sum = v;
        do {
            sum = (sum / 3) - 2;
            
            if (sum > 0)
                acc += sum;
        } while(sum > 0);

        return acc;
    });
}

int main() {
    auto input = readInput("input");

    std::cout<<"Part 1: "<<part1(input);
    std::cout<<"Part 2: "<<part2(input);

    return 0;
}
