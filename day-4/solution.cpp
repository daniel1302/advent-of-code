#include<iostream>
#include<fstream>
#include<string>
#include<vector>
#include<regex>
#include<unordered_map>

struct Guardian
{
    int id;
    std::array<int, 60> minutes{};
    int asleepTime = 0;
};


struct GuardianEvent
{
    struct {
        int y;
        int m;
        int d;
        int hr;
        int mi;
    } time;

    enum TYPE {
        SHIFT,
        AWAKE,
        ASLEEP
    } type;

    int guardianId;
};

std::vector<std::string> readInput(std::string fileName)
{
    std::ifstream input{fileName};
    std::vector<std::string> output{};
    std::string line;

    while(getline(input, line)) {
        output.push_back(line);
    }


    std::sort(output.begin(), output.end());
    return output;
}

std::vector<GuardianEvent> parseInput(std::vector<std::string>& input) {
    std::vector<GuardianEvent> output;
    const std::regex reShift("^\\[(\\d+)-(\\d+)-(\\d+) (\\d+):(\\d+)\\] Guard #(\\d+) begins shift");
    const std::regex reAwake("^\\[(\\d+)-(\\d+)-(\\d+) (\\d+):(\\d+)\\] wakes up");
    const std::regex reAsleep("^\\[(\\d+)-(\\d+)-(\\d+) (\\d+):(\\d+)\\] falls asleep");
    std::smatch match_results;


    GuardianEvent event;

    for (auto& item : input) {
        if (std::regex_match(item, match_results, reShift) && match_results.size() > 0) {
            event.type = GuardianEvent::TYPE::SHIFT;
            event.guardianId = std::stoi(match_results[6].str());
        } else if (std::regex_match(item, match_results, reAwake) && match_results.size() > 0) {
            event.type = GuardianEvent::TYPE::AWAKE;
            event.guardianId = 0;
        } else if (std::regex_match(item, match_results, reAsleep) && match_results.size() > 0) {
            event.type = GuardianEvent::TYPE::ASLEEP; 
            event.guardianId = 0;
        } else {
            continue;
        }

        event.time.y = std::stoi(match_results[1].str());
        event.time.m = std::stoi(match_results[2].str());
        event.time.d = std::stoi(match_results[3].str());
        event.time.hr = std::stoi(match_results[4].str());
        event.time.mi = std::stoi(match_results[5].str());
        output.push_back(event);
    }

    return output;
}

int step1(std::vector<GuardianEvent> events) 
{
    std::unordered_map<int, Guardian> guardians{};

    int currentGuardianId = 0;
    GuardianEvent& currentEvent;

    auto registerAsleep = [&](GuardianEvent& e1, GuardianEvent& e2) {
        
    };

    auto createGuardian = [&](int guardianId) {
        if (guardians.find(guardianId) != guardians.end()) {
            return;
        }

        guardians[guardianId] = Guardian();
    }
    

    for (auto& e : events) {
        if (currentGuardianId == 0) {
            currentGuardianId = e.guardianId;
            currentEvent = e;
            continue;
        }

        createGuardian(currentGuardianId);

        if (e.type == GuardianEvent::TYPE::AWAKE && currentState == GuardianEvent::TYPE::ASLEEP) {
            
        }

    }
}

int main()
{
    std::vector<std::string> inputLines = readInput("input");
    std::vector<GuardianEvent> events = parseInput(inputLines);
    std::cout<<step1(events);

    return 0;
}