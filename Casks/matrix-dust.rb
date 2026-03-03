cask "matrix-dust" do
  version "0.1.2"

  if Hardware::CPU.arm?
    sha256 "5364080a92f977d526dd152628c6727dadb01af4fde2cf742b3d570a1857e50c"
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_aarch64.dmg"
  else
    sha256 "9b950d38d8200884ce1836b19bd1103a998f1c4e80b874f52972776cf67dac9d"
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
