@rem Builds the pack into %output%.
@rem Run with `--check` to scan the copy.

ECHO OFF
ECHO "Building currently unavailable!"

@REM SET output=../SansFanficTerrariaAdventureRelease
@REM SET refs=Tools/generated_refs

@REM ECHO.
@REM ECHO "Building pack..."
@REM ECHO.

@REM Tools\t_pack_app.exe build -i . -o %output% -r %refs%

@REM if "%~1"=="--check" (GOTO CHECK) ELSE GOTO DONE

@REM :CHECK

@REM ECHO.
@REM ECHO "Scanning copy..."
@REM ECHO.

@REM Tools\t_pack_app.exe scan %output% %refs%

@REM :DONE
