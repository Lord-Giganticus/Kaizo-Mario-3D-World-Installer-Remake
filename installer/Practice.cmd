set out=%CD%
cd %1
cp %out%/Practice.ico %CD%
makensis CEMU-Practice.nsi
makensis SdCafiine-Practice-EUR.nsi
makensis SdCafiine-Practice-JPN.nsi
makensis SdCafiine-Practice-USA.nsi
makensis Practice.nsi
cp "Kaizo Mario 3D World Practice Mode.exe" %out%
rm Kaizo*.exe