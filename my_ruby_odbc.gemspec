# frozen_string_literal: true

require_relative "lib/my_ruby_odbc/version"

Gem::Specification.new do |spec|
  spec.name          = "my_ruby_odbc"
  spec.version       = MyRubyOdbc::VERSION
  spec.authors       = ["mnmandahalf"]
  spec.email         = ["manaminakamura812@gmail.com"]

  spec.summary       = "Ruby bindings for ODBC"
  spec.description   = "Ruby bindings for ODBC with native extension written in Rust"
  spec.homepage      = "https://github.com/mnmandahalf/my-ruby-odbc"
  spec.required_ruby_version = ">= 2.4.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/mnmandahalf/my-ruby-odbc"
  spec.metadata["changelog_uri"] = "https://github.com/mnmandahalf/my-ruby-odbc"

  spec.extensions = ["ext/my_rust_extension/extconf.rb"]
  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.glob("lib/**/*") +
             Dir.glob("ext/**/*") +
             Dir.glob("bin/**/*") +
             ["README.md", "Rakefile", "my_ruby_odbc.gemspec"]
  
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib", "ext"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, checkout our
  # guide at: https://bundler.io/guides/creating_gem.html
end
