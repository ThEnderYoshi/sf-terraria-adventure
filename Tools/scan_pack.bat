@rem Performs a TPack Toolbox scan and updates the README file.
@rem Run this from the repo's root dir.

echo off

echo.
echo === Scan Resource Pack ===
echo.

cd Tools
t_pack_app.exe scan .. tpack_refs --dump

echo.
echo === Update README ===
echo.

cd ..
py Tools/update_readme.py
