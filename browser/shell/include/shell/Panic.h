#pragma once

#include <exception>
#include <string_view>
namespace orbit::shell {
struct PanicException : std::exception {
 private:
  std::string_view message_;

 public:
  PanicException(std::string_view message) : message_(message) {}

  const char* what() const noexcept { return message_.data(); }
};
}  // namespace orbit::shell