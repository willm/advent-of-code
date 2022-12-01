#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <array>
using std::cout;
using std::string;
using std::vector;
using std::printf;

struct Counts {
    vector<string> zeros = {};
    vector<string> ones = {};
};

enum spread{minority, majority};

vector<string> read_lines(string file_path) {
    vector<string> input;
    std::ifstream file(file_path);
    if (file.is_open()) {
        while(file) {
            string line;
            std::getline(file, line);
            input.push_back(line);
        }
    }
    return input;
}

string get_binary_count(std::array<Counts, 12> counts, int majority, spread spread) {
    string binary_count = "";
    for (int i = 0; i < counts.size(); i++) {
        bool is_in_spread = spread == spread::majority ? counts[i].ones.size() > majority :
            counts[i].ones.size() < majority;
        if (is_in_spread) {
            binary_count.append("1");
        }
        else {
            binary_count.append("0");
        }
    }
    return binary_count;
}

unsigned long to_decimal(string binary) {
    return std::bitset<12>(binary).to_ulong();
}
unsigned long get_gamma_rate(std::array<Counts, 12> counts, int majority) {
    return to_decimal(get_binary_count(counts, majority, spread::majority));
}

unsigned long get_epsilon_rate(std::array<Counts, 12> counts, int majority)  {
    return to_decimal(get_binary_count(counts, majority, spread::minority));
}

vector<string> life_support_rating(vector<string> numbers, int pos, spread spread) {
    if (numbers.size() == 1 || pos == numbers[0].size()) {
        return numbers;
    }
    Counts counts = {};
    for (string num : numbers) {
        if (num[pos] == '1') {
            counts.ones.push_back(num);
        }
        else {
            counts.zeros.push_back(num);
        }
    }
    bool check = spread == spread::majority ? counts.ones.size() >= counts.zeros.size() :
        counts.ones.size() < counts.zeros.size();
    if (check) {
        return life_support_rating(counts.ones, ++pos, spread);
    }
    return life_support_rating(counts.zeros, ++pos, spread);
}

int main() {
    auto test_input = read_lines("./input-test.txt");
    auto oxy = life_support_rating(test_input, 0, spread::majority)[0];
    assert(oxy == "10111");
    assert(to_decimal(oxy) == 23);
    auto co2 = life_support_rating(test_input, 0, spread::minority)[0];
    assert(co2 == "01010");
    assert(to_decimal(co2) == 10);
    assert(to_decimal(co2) * to_decimal(oxy) == 230);


    auto input = read_lines("./input.txt");
    std::array<Counts, 12> counts;
    for (int i = 0; i < 12; i++) {
        counts[i] = {};
    }

    int majority = input.size() / 2;

    for (auto line : input) {
        for (int i = 0; i < line.length(); i++) {
            if (line[i] == '1') {
                counts[i].ones.push_back(line);
            }
            else {
                counts[i].zeros.push_back(line);
            }
        }
    }
    auto oxygen_rating = to_decimal(life_support_rating(input, 0, spread::majority)[0]);
    auto co2_rating = to_decimal(life_support_rating(input, 0, spread::minority)[0]);

    unsigned long power_consumption = get_gamma_rate(counts, majority) * get_epsilon_rate(counts, majority);
    assert(power_consumption == 2640986);
    printf(
        "power consumption %lu\noxygen %lu\nco2 %lu\nlife support %lu\n",
        power_consumption,
        oxygen_rating,
        co2_rating,
        oxygen_rating * co2_rating
    );

    return 0;
}
