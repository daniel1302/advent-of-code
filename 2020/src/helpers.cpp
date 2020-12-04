#include <vector>
#include <string>
#include <sstream>
#include <fstream>
#include <numeric>

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
    return std::accumulate(vec.begin(), vec.end(), std::string{}, [&delim](std::string& init, const typename T::value_type& item) -> std::string {
        return std::move(init) + ((init.length() > 0) ? delim : "") +  (std::stringstream{} << item).str();
    });
}