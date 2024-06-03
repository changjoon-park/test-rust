#include <stdio.h>
#include <stdlib.h>

void read_file_into_memory(const char *filename)
{
    FILE *file = fopen(filename, "rb");
    if (file == NULL)
    {
        printf("Could not open file %s\n", filename);
        return;
    }

    // Determine the file size
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    rewind(file);

    // Allocate memory to hold the file content
    char *buffer = (char *)malloc(file_size * sizeof(char));
    if (buffer == NULL)
    {
        printf("Memory allocation failed\n");
        fclose(file);
        return;
    }

    // Read the file into the buffer
    fread(buffer, sizeof(char), file_size, file);
    fclose(file);

    // Print the content of the file
    printf("File content:\n%s\n", buffer);

    // Free the allocated memory
    free(buffer);
}

int main()
{
    const char *filename = "example.txt";
    read_file_into_memory(filename);
    return 0;
}