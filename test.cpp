#include <nv_api.h>

#include <iostream>
#include <string>
int main() {
  nv::Context ctx;
  auto id = ctx.AddEntity();
  std::cout << "Hello C++" << std::endl;
  return 0;
}