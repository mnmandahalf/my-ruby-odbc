# frozen_string_literal: true

require_relative "my_ruby_odbc/version"
require_relative "../ext/my_rust_extension"

module ODBC
  class Error < StandardError; end
  # Your code goes here...
end
