#include <stdio.h>
#include <stdlib.h>
#include <string.h> // Include for strcpy

struct Student {
    char name[50];
    char id[10];
    int age;
    int grades[5];
};

int main()
{
    struct Student kevin;
    strcpy(kevin.name, "kevin park"); // Correct way to assign to char array
    strcpy(kevin.id, "ThisisID");     // Correct way to assign to char array
    kevin.age = 40;
    // Correct way to assign values to an int array
    kevin.grades[0] = 100;
    kevin.grades[1] = 90;
    kevin.grades[2] = 20;
    kevin.grades[3] = 30;
    kevin.grades[4] = 50;

    printf("Age: %d\n", kevin.age);

    return 1;
}
