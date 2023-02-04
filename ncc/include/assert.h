#ifndef __ASSERT_H__
#define __ASSERT_H__

// assert() macro
#ifndef NDEBUG
#define assert(test_val)\
if (!test_val)\
{\
    print_str("assertion failed");\
    print_endl();\
    asm () -> void { panic; } \
}
#else
#define assert(test_val) {}
#endif

#endif