#pragma once

#include <exception>
#include <string_view>
namespace orbit::window {
class GlfwFailed : std::exception {
 private:
  std::string_view msg;

 public:
  GlfwFailed(std::string_view msg) : msg(msg) {}
  const char* what() const noexcept override { return msg.data(); }
};

class Glfw {
 private:
  static void glfwErrorCallback(int error, const char* description);

 public:
  Glfw();
  ~Glfw();

  Glfw(const Glfw&);
  Glfw& operator=(const Glfw&);
  Glfw(Glfw&&);
  Glfw& operator=(Glfw&&);
};

}  // namespace orbit::window