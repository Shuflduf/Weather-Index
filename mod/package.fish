#!/usr/bin/env fish

echo "----- Compiling -----"
dotnet build -c Release

echo "----- Organizing -----"
mkdir ./tmp
mkdir ./tmp/WeatherIndex
set output_dir ./bin/Release/netstandard2.1

mv $output_dir/WeatherIndex.dll ./tmp/WeatherIndex/
mv $output_dir/icon_full.png ./tmp/
cp ./icon.png ./tmp/
cp ./manifest.json ./tmp/
cp ../README.md ./tmp/

echo "----- Zipping -----"
7z a -r WeatherIndex.zip ./tmp/*

echo "----- Cleaning Up -----"
rm -r ./tmp
