CPPFLAGS=-Wall -Wextra --std=c++2a -v
LDFLAGS=
LDLIBS=
TARGET=aoc

SRCS=src/main.cpp


build: 
	cppcheck src;
	g++ $(CPPFLAGS) -o aoc $(SRCS)