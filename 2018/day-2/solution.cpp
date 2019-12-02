#include<iostream>
#include<fstream>
#include<string>
#include<vector>
#include<thread>
#include<algorithm>
#include<array>
#include<optional>
#include<tuple>
	

std::vector<std::string> readInput(std::string fileName)
{
    std::ifstream input{fileName};
    std::vector<std::string> output{};
    std::string line;

    while(getline(input, line)) {
        output.push_back(line);
    }

    return output;
}

int calculateChecksum(std::vector<std::string>& input) 
{
    int amountDoubles = 0;
    int amountTriples = 0;


    for_each(input.begin(), input.end(), [&](std::string line){
        std::array<int, 255> characterSet{};
        std::for_each(line.begin(), line.end(), [&](char c) {
            characterSet[(int)c]++;            
        });

        bool foundTwoDuplications = false;
        bool foundThreeDuplications = false;
        std::for_each(characterSet.begin(), characterSet.end(), [&](int i) {
            foundTwoDuplications    |= (i == 2);
            foundThreeDuplications  |= (i == 3);
        });

        amountDoubles += static_cast<int>(foundTwoDuplications);
        amountTriples += static_cast<int>(foundThreeDuplications);
    });

    return amountDoubles * amountTriples;
}

int calculateDistance(std::string& str1, std::string& str2) 
{
    int distance = 0;
    int len = str1.size();
    if (len > str2.size()) {
        len = str2.size();
    }

    for (int i=0; i<len; i++) {
        distance += static_cast<int>(str1[i] != str2[i]);
    }

    return distance;
}

std::tuple<std::string, std::string> findCommonWords(std::vector<std::string>& input) 
{
    int len = input.size();
    int i, j, d;
    for (i=0; i<len-1; i++) {
        for (j=i+1; j<len;j++) {
            if (calculateDistance(input[i], input[j]) == 1) {
                return std::make_tuple(input[i], input[j]);
            }
        }
    }

    return std::make_tuple("", "");;
}

std::string getCommonCharacters(std::vector<std::string>& input)
{
    auto commonWords = findCommonWords(input);
    std::string word0 = std::get<0>(commonWords);
    std::string word1 = std::get<1>(commonWords);

    std::string output = std::string(word0.size(), '\0');
    auto it =  output.begin();
    for (int i=0;i<word0.size();i++) {
        if (word0[i] != word1[i]) {
            continue;
        }
        
        *(it++) = word0[i];
    }

    return output;
}

int main()
{
    std::vector<std::string> inputValues = readInput("input");
    std::cout<<calculateChecksum(inputValues)<<std::endl;
    std::cout<<getCommonCharacters(inputValues)<<std::endl;

    return 0;
}