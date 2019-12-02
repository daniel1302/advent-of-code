#include<iostream>
#include<string>
#include<fstream>
#include<vector>
#include<numeric>
#include<algorithm>
#include<set>

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

int sum(std::vector<int>& input)
{
    return std::accumulate(input.begin(), input.end(), 0);
}

bool found(int freq, std::set<int>& history) 
{
    return 
        history.end() != history.find(freq);
}

int findDuplication(std::vector<int>& input)
{
    std::set<int> history{};

    int freq = 0;
    int i = 0;
    int inputSize = input.size();

    while (true) {
        freq += input[(i++ % inputSize)];

        if (found(freq, history)) {
            break;
        }

        history.insert(freq);
    }

    return freq;
}

int main()
{
    auto inputValues = readInput("input");

    std::cout<<sum(inputValues)<<std::endl;
    
    std::cout<<findDuplication(inputValues);
}
