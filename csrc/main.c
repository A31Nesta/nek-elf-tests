extern int nek_add(int, int);

int main(void)
{
    volatile void *p = &nek_add;
    return p != 0;
}
