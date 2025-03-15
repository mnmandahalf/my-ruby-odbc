# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("my_ruby_odbc") do |ext|
  ext.ext_dir = "ext/my_rust_extension"
  ext.lib_dir = "lib/my_ruby_odbc"
end

task default: %i[clobber compile]
