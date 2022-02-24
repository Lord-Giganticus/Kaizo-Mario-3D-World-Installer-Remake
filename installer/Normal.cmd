set out=%CD%
cd %1
cp %OUT%/Normal.ico %CD%
makensis CEMU-Normal.nsi
makensis SdCafiine-Normal-EUR.nsi
makensis SdCafiine-Normal-JPN.nsi
makensis SdCafiine-Normal-USA.nsi
makensis Normal.nsi
cp "Kaizo Mario 3D World Normal Mode.exe" %out%
rm Kaizo*.exe