#include<iostream>
#include<vector>
#include<string>
#include<fstream>
#include<numeric>

std::string readInput(std::string fileName)
{
    std::ifstream input{fileName};
    std::string line;
    getline(input, line);

    return line;
}

std::vector<int> tokenize(const std::string str) 
{
    std::vector<int> out{};
    std::string token{};

    for (const char& c: str) {
        if (!token.empty() && (c < '0' || c > '9')) {
            
            out.push_back(atoi(token.c_str()));
            token.clear();

            continue;
        }

        token += c;
    }

    return out;
}

void f1(int a, int b, int o, std::vector<int>& mem) 
{
    if (a > mem.size() || b > mem.size()) {
        throw 1;
    }

    mem[mem[o]] = mem[mem[a]] + mem[mem[b]];
}

void f2(int a, int b, int o, std::vector<int>& mem) 
{
    if (a > mem.size() || b > mem.size()) {
        throw 1;
    }

    mem[mem[o]] = mem[mem[a]] * mem[mem[b]];
}

void compute(std::vector<int>& mem, int input1, int input2) 
{
    mem[1] = input1;
    mem[2] = input2;

    for (int pc=0; pc<mem.size(); pc+=4) {
        if (mem[pc] == 1) {
            f1(pc+1, pc+2, pc+3, mem);
        } else if (mem[pc] == 2) {
            f2(pc+1, pc+2, pc+3, mem);
        } else if (mem[pc] == 99) {
            break;
        } else {
            throw 2;
        }
    }
}

int part1(const std::vector<int>& mem)
{
    std::vector<int> memory = mem;
    compute(memory, 12, 2);

    return memory[0];
}

int part2(const std::vector<int>& mem, int expected)
{
    std::vector<int> memory{};
    for (int i1=0, i2; i1<mem.size(); i1++) {
        for (i2=0; i2<mem.size(); i2++) {
            memory = mem;
            compute(memory, i1, i2);

            if (memory[0] == expected) {
                return atoi((std::to_string(i1) + std::to_string(i2)).c_str());
            }
        }   
    }
    
    return -1;
}

int main() {
    auto memory = tokenize(readInput("input"));
    std::cout<<"Part 1: "<<part1(memory)<<"\n";
    std::cout<<"Part 2: "<<part2(memory, 19690720)<<"\n";
    
    return 0;
}
