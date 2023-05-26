#!/bin/sh
cargo build -r
sudo cp target/release/yumD /usr/bin/ 
echo "Build and binary file Copy Done"
sudo cp yumD.desktop /usr/share/applications/
echo "Desktop icon copy Done"
echo "Dir(binary:/usr/bin/, Desktop icon:/usr/share/applications/)"