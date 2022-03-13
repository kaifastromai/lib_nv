#include <cxx.h>
#include <lib.rs.h>

#include <iostream>
#include <string>
namespace nv {

class IComponent {};
// The current instance of novella
class Context {
  nvr::ContextInternal *ctx_;
  components::Field *field_;

 public:
  Context() { ctx_ = nvr::new_ctx(); }
  //Add a new entity. Returns the id of the entity.
  nvr::Id AddEntity() { return ctx_->add_entity(); }
  //Add a field component to an entity.
  void AddComponentField(nvr::Id entity, std::string field_name,
                         std::string field_value) {
    ctx_->add_field_component(entity, field_name, field_value);
  }
};
}  // namespace nv
