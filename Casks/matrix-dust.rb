cask "matrix-dust" do
  version "0.1.3"

  if Hardware::CPU.arm?
    sha256 "0431ffb08b857a5feca3aa5dac6200292e998fe16225b63d8b46d6f22ec8595f"
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_aarch64.dmg"
  else
    sha256 "4c0964dfcb5dab42c00f10c507f9bc6636a027190adcff9d1ceb68d1f9759673"
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_x64.dmg"
  end

  name "Matrix Dust"
  desc "Lightweight macOS window layout manager with global keyboard shortcuts"
  homepage "https://github.com/ZouYouShun/matrix-dust"

  app "Matrix Dust.app"

  zap trash: [
    "~/Library/Application Support/com.matrix-dust.matrix-dust",
    "~/Library/Preferences/com.matrix-dust.matrix-dust.plist",
    "~/Library/Logs/com.matrix-dust.matrix-dust",
  ]
end
