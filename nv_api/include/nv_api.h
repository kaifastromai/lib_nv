#include "nv_api/src/lib.rs.h"
#include "rust/cxx.h"
namespace nv {
class Context {
  ContextInternal *ctx_;

 public:
  Context();
  void sayHello();
};

Context::Context() { ctx_ = new_ctx(); };
void Context::sayHello() { std::cout << "Hello from C++" << std::endl; }

}  // namespace nv