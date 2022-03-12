#include <cxx.h>
#include <lib.rs.h>

#include <iostream>
#include <string>
namespace nv {

class Context {
  nvr::ContextInternal *ctx_;

 public:
  Context();
  nvr::Id AddEntity();
  void AddComponentField(std::string entity, std::string field_name,
                         std::string field_value);
};

Context::Context() { ctx_ = nvr::new_ctx(); };

void Context::AddComponentField(std::string entity, std::string field_name,
                                std::string field_value) {
  ctx_->add_field_component(entity, field_name, field_value);
}
nvr::Id Context::AddEntity() { return ctx_->add_entity(); 
}

}  // namespace nv
