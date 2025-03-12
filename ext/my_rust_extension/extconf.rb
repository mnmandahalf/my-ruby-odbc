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

system("cargo build --release --target #{rust_target}") || abort("Cargo build failed")

lib_path = File.join("target", "#{rust_target}", "release", "libmy_rust_extension.so")

unless File.exist?(lib_path)
  abort("no library found: #{lib_path}")
end

FileUtils.cp(lib_path, File.join(__dir__, "my_rust_extension.so"))

# this is dummy Makefile
create_makefile("my_rust_extension")
