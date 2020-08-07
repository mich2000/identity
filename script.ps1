Set-Location .\identity_web
cargo update
Set-Location ..
git add .
git commit -m $args[0]
git push origin master