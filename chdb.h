#pragma once

#include <stddef.h>
#include <stdint.h>

struct local_result_v2
{
    char* buf; // plaint output, such as CSV, JSON
    size_t len; // bytes length
    void* _vec; // std::vector<char> *, for freeing
    double elapsed;
    uint64_t rows_read;
    uint64_t bytes_read;
    char* error_message; // error message
};

struct local_result_v2* query_stable_v2(int argc, char** argv);
void free_result_v2(struct local_result_v2* result);