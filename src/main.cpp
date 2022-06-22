//
// Created by linjiaming on 2/25/2022.
//

#include <fstream>
#include <iostream>

int main() {
    std::ifstream f;
    f.open("aaaaaaaa");
    std::string s;
    std::getline(f, s);
    std::cout << s << std::endl;
    return 0;
}