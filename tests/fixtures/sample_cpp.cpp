int x = 1; // NOLINT
int y = 2; // NOLINTNEXTLINE
int z = 3;
// NOLINTBEGIN
int w = 4;
// NOLINTEND
// cppcheck-suppress uninitvar
int v;
