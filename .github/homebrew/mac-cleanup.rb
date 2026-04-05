# Reference copy of the Homebrew formula for this project.
#
# This file is NOT used by any workflow here — it exists as the canonical
# template for the `Yurzs/homebrew-tap` repo. Copy it to
# `Formula/mac-cleanup.rb` in that repo on first setup. After that, the
# `update-tap` job in `.github/workflows/release.yml` keeps the version and
# sha256 fields in the tap copy in sync on every tagged release.
class MacCleanup < Formula
  desc "macOS disk cleanup tool for developers"
  homepage "https://github.com/Yurzs/mac-cleanup"
  version "0.2.1"
  license "MIT"

  # Universal binary handles both arm64 and x86_64 macOS.
  url "https://github.com/Yurzs/mac-cleanup/releases/download/v#{version}/mac-cleanup-universal-apple-darwin.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000"

  depends_on :macos

  def install
    bin.install "mac-cleanup"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/mac-cleanup --version")
  end
end
