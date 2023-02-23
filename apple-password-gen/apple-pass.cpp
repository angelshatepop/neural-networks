#include <iostream>
#include <random>
#include <string>
#include <fstream>
using namespace std;

int main()
{
    random_device device;
    mt19937_64 rng(device());
    uniform_int_distribution<std::mt19937_64::result_type> intdist(0,9);
    uniform_int_distribution<std::mt19937_64::result_type> chardistUpper(65,90);
    uniform_int_distribution<std::mt19937_64::result_type> chardistLower(97,122);
    uniform_int_distribution<std::mt19937_64::result_type> choicedist(1,3);

    string passphrase = "";
    passphrase += char(chardistUpper(rng));

    int i = 2;
    while(i <= 24)
    {
        while(i%8 != 0)
        {
            switch(choicedist(rng))
            {
                case 1:
                    passphrase += std::to_string(intdist(rng));
                    break;
                case 2:
                    passphrase += char(chardistLower(rng));
                    break;
                default:
                    passphrase += char(chardistUpper(rng));
                    break;
            }
            break;
        }
        if(i%8 == 0 && i<20)
        {
            passphrase += "-";
        }
        i++;
    }
    cout << passphrase;
    return 0;
}