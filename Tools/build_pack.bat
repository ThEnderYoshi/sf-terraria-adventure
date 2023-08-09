:: Builds the pack into %output%.
:: Run with `--check` to scan the copy.

ECHO OFF
SET output=../SansFanficTerrariaAdventureRelease
SET refs=Tools/generated_refs

ECHO.
ECHO "Building pack..."
ECHO.

Tools\t_pack_diagnostic.exe build -i . -o %output% -r %refs%

if "%~1"=="--check" (GOTO CHECK) ELSE GOTO DONE

:CHECK

ECHO.
ECHO "Scanning copy..."
ECHO.

Tools\t_pack_diagnostic.exe scan -i %output% -r %refs%

:DONE
