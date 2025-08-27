#include "window/Glfw.h"

#include <atomic>
#include <iostream>

#include "GLFW/glfw3.h"

namespace orbit::window {
static std::atomic<uint64_t> GLFW_REF_COUNTER = 0;

void Glfw::glfwErrorCallback(int error, const char* description) {
  std::cerr << "GLFW Error [" << error << "]: " << description << "\n";
}

Glfw::Glfw() {
  if (GLFW_REF_COUNTER.fetch_add(1) == 0) {
    glfwSetErrorCallback(glfwErrorCallback);
    if (glfwInit() == GLFW_FALSE) {
      throw GlfwFailed("Failed to initialize GLFW");
    }
  }
}

Glfw::~Glfw() {
  if (GLFW_REF_COUNTER.fetch_sub(1) == 1) {
    glfwTerminate();
  }
}

Glfw::Glfw(const Glfw&) : Glfw() {}
Glfw& Glfw::operator=(const Glfw&) { return *this; }
Glfw::Glfw(Glfw&&) = default;
Glfw& Glfw::operator=(Glfw&&) = default;

}  // namespace orbit::window