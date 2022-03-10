
#include "./nv_api_internal.h"
namespace nv {
// Context represents a specific instance of the Novella engine. There should be
// only one active at any given time
class Context {
  Mir* mir;

 public:
  Context() { context = nv_internal::create(); }

  ~Context() { nv_internal::destroy(&context); }
};
}  // namespace nv