class Sentinel < Formula
  desc "System-level CLI ad blocker with safe recovery workflows"
  homepage "https://github.com/sentinel-cli/sentinel-cli"
  version "__VERSION__"
  url "__ARCHIVE_URL__"
  sha256 "__SHA256__"

  def install
    bin.install "sentinel"
  end

  test do
    system "#{bin}/sentinel", "--help"
  end
end
