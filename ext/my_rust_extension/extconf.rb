# frozen_string_literal: true

require 'mkmf'
require "rbconfig"

target_cpu = RbConfig::CONFIG["target_cpu"]
rust_target = case target_cpu
              when "x86_64"
                "x86_64-apple-darwin"
              when "arm64", "aarch64"
                "aarch64-apple-darwin"
              else
                abort("Unsupported target CPU: #{target_cpu}")
              end

ENV["ARCHFLAGS"] = "-arch #{target_cpu}"

# Ensure we're using the correct Rust toolchain for the target architecture
system("cargo build --release --target #{rust_target}") || abort("Cargo build failed")

# Determine library extension based on platform
lib_extension = case RbConfig::CONFIG['host_os']
when /darwin|mac os/i
  "dylib"
when /linux/
  "so"
when /mswin|mingw|cygwin/
  "dll"
else
  abort("Unsupported platform: #{RbConfig::CONFIG['host_os']}")
end

# On macOS, the extension is .dylib
lib_path = File.join("target", "#{rust_target}", "release", "libmy_rust_extension.#{lib_extension}")
unless File.exist?(lib_path)
  abort("no library found: #{lib_path}")
end

# Get platform-specific extension
dlext = RbConfig::CONFIG['DLEXT'] # 'bundle' on macOS, 'so' on Linux, 'dll' on Windows

extension_path = File.join(__dir__, "my_rust_extension.#{dlext}")

FileUtils.cp(lib_path, extension_path)

# this is dummy Makefile
create_makefile("my_rust_extension")
