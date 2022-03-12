#include <nv_api.h>

#include <iostream>
#include <string>
int main() {
  nv::Context ctx;
  ctx.sayHello();
  ctx.say_hello_rust();
  std::cout << "Hello C++" << std::endl;
  return 0;
}