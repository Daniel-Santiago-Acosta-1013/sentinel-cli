class Sentinel < Formula
  desc "System-level CLI ad blocker with safe recovery workflows"
  homepage "https://github.com/Daniel-Santiago-Acosta-1013/sentinel-cli"
  version "0.2.0"
  url "__ARCHIVE_URL__"
  sha256 "__SHA256__"

  def install
    bin.install "sentinel"
  end

  test do
    system "#{bin}/sentinel", "--help"
  end
end
