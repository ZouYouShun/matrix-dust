cask "window-tuner" do
  version "0.1.0"

  if Hardware::CPU.arm?
    sha256 :no_check
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_aarch64.dmg"
  else
    sha256 :no_check
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_x64.dmg"
  end

  name "Matrix Dust"
  desc "Lightweight macOS window layout manager with global keyboard shortcuts"
  homepage "https://github.com/ZouYouShun/matrix-dust"

  app "Matrix Dust.app"

  zap trash: [
    "~/Library/Application Support/com.matrix-dust.window-tuner",
    "~/Library/Preferences/com.matrix-dust.window-tuner.plist",
    "~/Library/Logs/com.matrix-dust.window-tuner",
  ]
end
