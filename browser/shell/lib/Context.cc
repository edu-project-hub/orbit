#include "shell/Context.h"

#include <GLFW/glfw3.h>

#include <atomic>
#include <cstddef>

#include "shell/Panic.h"

namespace orbit::shell {
static std::atomic<std::size_t> GLFW_REF_COUNT = 0;

Glfw::Glfw() {
  if (GLFW_REF_COUNT.fetch_add(1, std::memory_order_acq_rel) == 0) {
    if (glfwInit() == GLFW_FALSE) {
      throw PanicException("Failed to initialize GLFW");
    }
  }
}

Glfw::~Glfw() {
  if (GLFW_REF_COUNT.fetch_sub(1, std::memory_order_acq_rel) == 1) {
    glfwTerminate();
  }
}

Glfw::Glfw(const Glfw&) : Glfw() {}
Glfw& Glfw::operator=(const Glfw& other) {
  if (this != &other) {
    this->~Glfw();
    new (this) Glfw(other);
  }
  return *this;
}

}  // namespace orbit::shell