#include<string>
#include<vector>
#include<iostream>
#include<fstream>
#include<algorithm>

std::string readInput(std::string fileName)
{
    std::ifstream input{fileName};
    std::string line;

    getline(input, line);

    return line;
}


std::pair<int, int> solution(std::string& inputStr) 
{
   
    auto polarity = [](char c) -> char {
        return (c <= 'Z') ? c + 32 : c - 32; 
    };

    auto lengthWithoutPolymers = [](std::string& inputStr, auto polarity) -> int {
        int i;
        int str;
        bool removed = false;

        do {
            removed = false;

            str = inputStr.size();
            for (i=0; i<str-1; i++) {
                if (inputStr[i+1] == polarity(inputStr[i])) {
                    inputStr[i] = '-';
                    inputStr[i+1] = '-';
                    i++;

                    removed = true;
                }
            }
            
            inputStr.erase(std::remove(inputStr.begin(), inputStr.end(), '-'), inputStr.end());
        } while(removed == true);

        return inputStr.size();
    };

    auto lengthWithoutChar = [](std::string inputStr, char c, auto lengthWithoutPolymers, auto polarity) -> int {
        inputStr.erase(std::remove(inputStr.begin(), inputStr.end(), c), inputStr.end());
        inputStr.erase(std::remove(inputStr.begin(), inputStr.end(), polarity(c)), inputStr.end());

        return lengthWithoutPolymers(inputStr, polarity);
    };
    
    std::vector<int> lengths{};
    for (char j='a'; j<='z'; j++) {
        lengths.push_back(lengthWithoutChar(inputStr, j, lengthWithoutPolymers, polarity));
    }
    
    return std::make_pair(
        lengthWithoutPolymers(inputStr, polarity),
        *(std::min_element(lengths.begin(), lengths.end()))
    );
}

int main()
{
    std::string inputString = readInput("input");
    auto result = solution(inputString);
    std::cout<<result.first<<std::endl<<result.second<<std::endl;

    return 0;
}