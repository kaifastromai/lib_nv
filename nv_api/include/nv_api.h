#include <cxx.h>
#include <lib.rs.h>

#include <iostream>
#include <string>
#include <vector>
namespace nv {
// The current instance of novella
class Context {
  nvr::ContextInternal *ctx_;

 public:
  Context() { ctx_ = nvr::new_ctx(); }
  // Add a new entity. Returns the id of the entity.
  nvr::Id AddEntity() { return ctx_->add_entity(); }
  // Add a field component to an entity.
  void AddComponentField(nvr::Id entity, std::string field_name,
                         std::string field_value) {
    ctx_->add_field_component(entity, field_name, field_value);
  }
  rust::cxxbridge1::Vec<nvr::Id> GetAllLivingEntities() {
    return ctx_->get_all_living_entities();
  }
  ~Context() { nvr::drop(ctx_); }
};
}  // namespace nv
