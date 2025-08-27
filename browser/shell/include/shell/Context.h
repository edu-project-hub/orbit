#pragma once

namespace orbit::shell {

class Glfw {
 public:
  Glfw();
  ~Glfw();

  Glfw(const Glfw&);
  Glfw& operator=(const Glfw&);

  Glfw(Glfw&&) noexcept = default;
  Glfw& operator=(Glfw&&) noexcept = default;
};
}  // namespace orbit::shell