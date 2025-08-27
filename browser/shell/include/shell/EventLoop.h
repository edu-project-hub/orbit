#pragma once

#include <GLFW/glfw3.h>

#include <cstddef>
#include <unordered_map>

#include "shell/Context.h"
namespace orbit::shell {
struct WindowId {
  size_t id;
};

class EventLoop {
 private:
  [[maybe_unused]] Glfw glfw_;  // ensure it lives long enough
  std::unordered_map<WindowId, GLFWwindow*> windows_;
  
};
}  // namespace orbit::shell