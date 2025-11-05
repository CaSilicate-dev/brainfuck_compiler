
        #include <stdio.h>
        #include <stdlib.h>
        #define TAPE_SIZE 1000000

        int main(){unsigned char tape[TAPE_SIZE] = {0};unsigned char *ptr = tape;
    ++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;while (*ptr){++ptr;++*ptr;++*ptr;++*ptr;++*ptr;while (*ptr){++ptr;++*ptr;++*ptr;++ptr;++*ptr;++*ptr;++*ptr;++ptr;++*ptr;++*ptr;++*ptr;++ptr;++*ptr;--ptr;--ptr;--ptr;--ptr;--*ptr;}++ptr;++*ptr;++ptr;++*ptr;++ptr;--*ptr;++ptr;++ptr;++*ptr;while (*ptr){--ptr;}--ptr;--*ptr;}++ptr;++ptr;putchar(*ptr);++ptr;--*ptr;--*ptr;--*ptr;putchar(*ptr);++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;++*ptr;putchar(*ptr);putchar(*ptr);++*ptr;++*ptr;++*ptr;putchar(*ptr);++ptr;++ptr;putchar(*ptr);--ptr;--*ptr;putchar(*ptr);--ptr;putchar(*ptr);++*ptr;++*ptr;++*ptr;putchar(*ptr);--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;putchar(*ptr);--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;--*ptr;putchar(*ptr);++ptr;++ptr;++*ptr;putchar(*ptr);++ptr;++*ptr;++*ptr;putchar(*ptr);return 0;}
    