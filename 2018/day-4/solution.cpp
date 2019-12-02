#include<iostream>
#include<fstream>
#include<string>
#include<vector>
#include<regex>
#include<unordered_map>
#include<algorithm>


struct Guardian
{
    int id;
    std::array<int, 60> minutes{};
    int asleepTime = 0;
    std::pair<int, int> mostCommonMinute;
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

std::pair<int, int> solution(std::vector<GuardianEvent>& events)
{
    std::unordered_map<int, Guardian> guardians{};

    int currentGuardianId = 0;
    GuardianEvent& currentEvent = events[0];

    auto registerAsleep = [&](Guardian& guardian, GuardianEvent& e1, GuardianEvent& e2) {
        // std::cout<<"("<<guardianId<<")"<<e1.time.hr<<":"<<e1.time.mi<<" - "<<e2.time.hr<<":"<<e2.time.mi<<std::endl;

        int start  = std::max({0, e1.time.mi});
        int finish = std::min({59, e2.time.mi});

        guardian.asleepTime += (finish + 1 - start);

        for (int t=start; t<finish; t++) {
            guardian.minutes[t]++;
        }
    };

    auto getGuardian = [&](int guardianId) -> Guardian& {
        auto it = guardians.find(guardianId);

        if (it != guardians.end()) {
            return (it->second);
        }

        guardians[guardianId] = Guardian();
        guardians[guardianId].id = guardianId;

        return guardians[guardianId];
    };




    auto findTheMostCommonMinute = [](Guardian& guardian) -> std::pair<int, int> {

        auto maxElement = std::max_element(guardian.minutes.begin(), guardian.minutes.end());

        return std::make_pair(
            maxElement - guardian.minutes.begin(),
            *maxElement
        );
    };

    auto findTheMostRegularPerson = [](std::unordered_map<int, Guardian>& guardians) -> Guardian {
        Guardian& _guardian = guardians[0];

        for (auto& g : guardians) {
            if (g.second.mostCommonMinute.second > _guardian.mostCommonMinute.second) {
                _guardian = g.second;
            }
        }

        return _guardian;
    };

    auto findLeziesGuardian = [](std::unordered_map<int, Guardian>& guardians) -> Guardian {

        Guardian& guardian = guardians[0];

        for (auto& g : guardians) {
            if (g.second.asleepTime > guardian.asleepTime) {
                guardian = g.second;
            }
        }

        return guardian;
    };


    for (auto& e : events) {
        if (currentGuardianId == 0) {
            currentGuardianId = e.guardianId;
            currentEvent = e;
            continue;
        }

        if (e.type == GuardianEvent::TYPE::AWAKE && currentEvent.type == GuardianEvent::TYPE::ASLEEP) {
            registerAsleep(getGuardian(currentGuardianId), currentEvent, e);
        }

        currentEvent = e;

        if (e.type == GuardianEvent::TYPE::SHIFT) {
            currentGuardianId = e.guardianId;
        }
    }

    for (auto& g : guardians) {
        g.second.mostCommonMinute = findTheMostCommonMinute(g.second);
    }


    Guardian theLeziestGuardian = findLeziesGuardian(guardians);
    Guardian theMostRegularGuardian = findTheMostRegularPerson(guardians);


    return std::make_pair(
        theLeziestGuardian.mostCommonMinute.first*theLeziestGuardian.id,
        theMostRegularGuardian.mostCommonMinute.first*theMostRegularGuardian.id
    );
}

int main()
{
    std::vector<std::string> inputLines = readInput("input");
    std::vector<GuardianEvent> events = parseInput(inputLines);
    auto results = solution(events);

    std::cout<<results.first<<std::endl<<results.second<<std::endl;


    return 0;
}