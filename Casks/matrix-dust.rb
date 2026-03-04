cask "matrix-dust" do
  version "0.1.4"

  if Hardware::CPU.arm?
    sha256 "c129f912ffddea48be677fa2f007e72201f1ea065c97d459a020c5b5fb83e466"
    url "https://github.com/ZouYouShun/matrix-dust/releases/download/v#{version}/Matrix.Dust_#{version}_aarch64.dmg"
  else
    sha256 "3c7ba6eaa64e08d8bfecbfbeec251c5a5ac330f9668a60de70aa7972ab2fbaff"
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
