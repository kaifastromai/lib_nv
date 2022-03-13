#include <nv_api.h>

#include <iostream>
#include <string>
int main() {
  nv::Context ctx;
  auto id = ctx.AddEntity();
  ctx.AddComponentField(id, "name", "foo");
  std::cout << "The id of the entity is: " << id.to_string() << std::endl;
  return 0;
}