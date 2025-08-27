#pragma once

#include "window/Glfw.h"
namespace orbit::window {
class Application {
 public:
  
};

class EventLoop {
 private:
  Glfw glfw_;

 public:
  EventLoop() = default;
};
}  // namespace orbit::window