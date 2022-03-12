#include <cxx.h>
#include <lib.rs.h>

#include <iostream>
namespace nv {
class Context {
  ContextInternal *ctx_;

 public:
  Context();
  void sayHello();
  void say_hello_rust();
};

Context::Context() { ctx_ = new_ctx(); };
void Context::sayHello() { std::cout << "Hello C++" << std::endl; }

void Context::say_hello_rust() { say_hello(); }

}  // namespace nv
