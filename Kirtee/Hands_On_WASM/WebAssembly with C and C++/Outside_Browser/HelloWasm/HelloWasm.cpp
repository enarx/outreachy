#include <iostream>
#include <string>

int main()
{
    std::string firstname;
    std::string compiler;
    std::cout << "Hello User, Enter your first name.\n ";
    std::cin >> firstname;
    std::cout << "Enter your compiler name you gonna work with now.(gcc/wasienv)\n ";
    std::cin >> compiler;
    std::cout << "Hello " << firstname << ". you ran this program compiled through "<< compiler;
}