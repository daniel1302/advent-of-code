#include <string>
#include<iostream>


#include "helpers.cpp"


int day1_1(const std::string& filename) {
    auto input = loadIntegers(filename);

    for (unsigned long i=0; i<input.size(); i++) {
        for (unsigned long j=i+1; j<input.size(); j++) {
            if (input[i] + input[j] == 2020) {
                return input[j] * input[i];
            }
        }
    }

    return -1;
}

int day1_2(const std::string& filename) {
    auto input = loadIntegers(filename);

    for (unsigned long i=0; i<input.size(); i++) {
        for (unsigned long j=i+1; j<input.size(); j++) {
            for (unsigned long k=j+1; k<input.size(); k++) {
                if (input[i] + input[j] + input[k] == 2020) {
                    return input[j] * input[i] * input[k] ;
                }
            }
        }
    }

    return -1;
}