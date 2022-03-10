#include "nv_api/src/lib.rs.h"
#include "rust/cxx.h"
namespace nv {
class Context {
  ContextInternal *ctx_;

 public:
  Context();
  void sayHello();
};
}  // namespace nv