#include <numeric>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <ranges>
#include <map>

int main()
{
    auto input = "./sample_input/01a";
    std::string line;
    std::fstream file(input);

    std::vector<int> vector1;
    std::vector<int> vector2;
    while (std::getline(file, line))
    {
        auto delimiter_position = line.find("   ");
        auto a = line.substr(0, delimiter_position);
        auto b = line.substr(delimiter_position + 3, line.length());
        vector1.push_back(std::stoi(a));
        vector2.push_back(std::stoi(b));
    }

    std::sort(vector1.begin(), vector1.end());
    std::sort(vector2.begin(), vector2.end());
    
    int result = 0;
    for (auto [elem1, elem2] : std::views::zip(vector1, vector2))
        result += abs(elem2 - elem1);
    std::cout << result << "\n";

    int part_2_result = 0;
    std::map<int, int> vec_2_counter;
    for (const int& right : vector2)
    {
        if (vec_2_counter.contains(right))
            vec_2_counter[right]++;
        else
            vec_2_counter[right] = 1;
    }
    for (const int& left : vector1)
    {
        if (vec_2_counter.contains(left))
            part_2_result += left * vec_2_counter[left];
    }
    std::cout << part_2_result;
}