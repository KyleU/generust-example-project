class generust_example_project < Formula
  version 'v0.0.40'
  desc "A work in progress."
  homepage "https://github.com/generust-example-project/generust-example-project"

  if OS.mac?
      url "https://github.com/generust-example-project/generust-example-project/releases/download/#{version}/generust-example-project-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "e06290a647306d92f1a20c9903060814f6938099a08437b929b32c256aa1dc2c"
  elsif OS.linux?
      url "https://github.com/generust-example-project/generust-example-project/releases/download/#{version}/generust-example-project-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b630ba7bb62902866a0252c4187188d92c67ae2ac92468d3a2d418d680e25364"
  end

  def install
    bin.install "generust-example-project"
  end
end

