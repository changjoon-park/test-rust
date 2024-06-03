#include <stdio.h>

void array_example()
{
    char s1[10]; // Static array
    s1[0] = 'A';
    s1[1] = 'B';
    s1[2] = '\0'; // Null-terminate for printing as a string

    printf("Array s1: %s\n", s1);
    printf("Address of s1: %p\n", (void *)s1);
}

int main()
{
    array_example();
    return 0;
}
