@rem Creates an optimized copy of the pack on the parent dir.
@rem Run this from the root dir.

echo off

echo.
echo "Build pack"
echo.

Tools\pack_crusher.exe . ../SansFanficTerrariaAdventure_Release
