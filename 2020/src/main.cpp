
#include <cstdlib>
#include <iostream>
#include "day1.cpp"


int main(int argc, char *argv[]) {
    int day;
    if (argc < 2) {
        std::cerr<<"Udage: aoc day_no";
        return 1;
    }

    day = std::atoi(argv[1]);

    switch (day) {
        case 1:
            std::cout<<"Part 1: "<<day1_1("data/day1_1.bin")<<"\n";
            std::cout<<"Part 2: "<<day1_2("data/day1_2.bin")<<"\n";
            break;
        default:
            std::cerr<<"Action not supported";
            break;
    }

    return 0;
}