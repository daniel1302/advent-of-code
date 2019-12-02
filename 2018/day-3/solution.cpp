#include<iostream>
#include<fstream>
#include<string>
#include<vector>
#include<unordered_map>
#include<regex>
#include<algorithm>

struct Area
{
    int id;
    int x;
    int y;
    int width;
    int length;
    bool duplicated = false;
};

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

std::vector<Area> parseInput(std::vector<std::string>& inputValues)
{
    std::vector<Area> output{};
    std::regex re( "^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)" );
    std::smatch match_results;

    auto p = Area();

    for (auto& input : inputValues) {
        if(!std::regex_match(input, match_results, re) || match_results.size() <= 0) {
            continue;
        }

        p.id        = std::stoi(match_results[1].str());
        p.x         = std::stoi(match_results[2].str());
        p.y         = std::stoi(match_results[3].str());
        p.width     = std::stoi(match_results[4].str());
        p.length    = std::stoi(match_results[5].str());

        output.push_back(p);
    }

    return output;
}


std::pair<int, int> countOverLap(std::vector<Area>& input)
{
    std::unordered_map<std::string, int> usedSquare{};
    std::string hash;
    int x, y;

    auto iterateOverInput = [&](auto insideFunc) {
        for (auto& item : input) {
            for (x=item.x; x<item.x+item.width; x++) {
                for (y=item.y; y<item.y+item.length; y++) {
                    insideFunc(x, y, item);
                }
            }
        }
    };

    iterateOverInput([&](int x, int y,  Area& item){
        hash = std::to_string(x) + ":" + std::to_string(y);
        if (usedSquare.find(hash) != usedSquare.end()) {
            usedSquare.insert({hash, 0});
        }

        usedSquare[hash] += 1;
    });

    int lastItemId = 0;
    iterateOverInput([&](int x, int y, Area& item) {
        if (item.duplicated) {
            return;
        }

        hash = std::to_string(x) + ":" + std::to_string(y);
        if (usedSquare[hash] > 1) {
            item.duplicated = true;
        }
    });



    int duplicated = count_if(usedSquare.begin(), usedSquare.end(), [](std::pair<std::string, int> item){
        return item.second > 1;
    });

    auto nonOverlaped = find_if(input.begin(), input.end(), [](Area& item) {
        return item.duplicated == false;
    });

    return std::make_pair(duplicated, (*nonOverlaped).id);
}


//118322
int main()
{
    std::vector<std::string> inputValues = readInput("input");
    std::vector<Area> areas = parseInput(inputValues);
    auto solution = countOverLap(areas);

    std::cout<<solution.first<<std::endl<<solution.second<<std::endl;



    return 0;
}
