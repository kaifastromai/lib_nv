#include "nv_api/include/nv_api.h"

#include <iostream>
#include <string>

namespace nv {

Context::Context() { ctx_ = new_ctx(); };
void Context::sayHello() { std::cout << "Hello from C++" << std::endl; }

}  // namespace nv