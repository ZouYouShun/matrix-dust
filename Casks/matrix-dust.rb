cask "matrix-dust" do
  version "0.1.0"

  if Hardware::CPU.arm?
    sha256 "c06b31069b6d5e81c935c02d4cd9e73da59ed72c2b54a3d43efa63dfae07f05d"
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_aarch64.dmg"
  else
    sha256 "bb6a2ff5e8fafcf79d19fe0cab45f8085043c2f080d4b12e935fb156279d1fe9"
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
