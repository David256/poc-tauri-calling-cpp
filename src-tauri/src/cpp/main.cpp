// src/cpp/main.cpp
#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
#include <cstring>

using namespace std;

extern "C"
{
    int test_cpp(int *vec, int lenght)
    {
        int result = 0;

        for (int i = 0; i < lenght; i++)
        {
            cout << vec[i] << endl;
            result += vec[i];
        }

        return result;
    }
    int raw_sum_array_of_numbers(vector<int> input)
    {
        int result = 0;

        for (size_t i = 0; i < input.size(); i++)
        {
            result += input[i];
        }

        return result;
    }

    char **raw_convert_each_value_to_uppercase(char **vec, int lenght)
    {
        for (int i = 0; i < lenght; i++)
        {
            cout << vec[i] << endl;
            for (size_t j = 0; j < strlen(vec[i]); j++)
            {
                vec[i][j] = toupper(vec[i][j]);
            }
            cout << vec[i] << endl;
        }

        return vec;
    }
}
