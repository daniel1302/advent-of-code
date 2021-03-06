#pragma once


#include <string>
#include <regex>
#include <algorithm>
#include <vector>
#include <fstream>
#include <sstream>
#include <numeric>
#include <iostream>
#include <iterator>

std::vector<int> loadIntegers(const std::string& filename) {
    std::ifstream input{filename};
    std::vector<int> output{};
    std::string line;
    while(getline(input, line)) {
        output.push_back(std::stoi(line));
    }

    return output;
}

template <typename T>
std::string join(const T& vec, std::string delim) {
    return std::accumulate(vec.begin(), vec.end(), std::string{}, 
        [&delim](std::string init, const typename T::value_type& item) -> std::string {
            return std::move(init) 
                + ((init.length() > 0) ? delim : "") 
                + (std::stringstream{item}).str();
        }
    );
}




std::vector<std::string> tokenize(const std::string& input, const std::string& token) {
    std::vector<std::string> res{};

    std::regex tokenRegex{token};
    std::copy(
        std::sregex_token_iterator{input.begin(), input.end(), tokenRegex, -1},
        std::sregex_token_iterator{},
        std::back_inserter(res)       
    );

    return res;
}