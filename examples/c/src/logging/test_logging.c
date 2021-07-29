// #include "logging.c"
// #include <criterion/criterion.h>

// Test(logging, sanity) {
//   uint8_t instruction_data[] = {10, 11, 12, 13, 14};
//   SafePubkey program_id = {.x = {
//                               1,
//                           }};
//   SafePubkey key = {.x = {
//                        2,
//                    }};
//   uint64_t lamports = 1;
//   uint8_t data[] = {0, 0, 0, 0};
//   SafeAccountInfo accounts[] = {};
//   SafeParameters params = {accounts, sizeof(accounts) /
//   sizeof(SafeAccountInfo), instruction_data,
//                           sizeof(instruction_data), &program_id};

//   cr_assert(SUCCESS == logging(&params));
// }
