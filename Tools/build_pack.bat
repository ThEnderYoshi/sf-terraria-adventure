:: Builds the pack into %output%.
:: Run with `--check` to scan the copy.

ECHO OFF
SET output=../../../SansFanficTerrariaAdventureRelease

ECHO.
ECHO "Building pack..."
ECHO.

CD Tools\pack_diagnostic
cargo run -- build -i ../.. -o %output% -r ../generated_refs

if "%~1"=="--check" (GOTO CHECK) ELSE GOTO DONE

:CHECK

ECHO.
ECHO "Scanning copy..."
ECHO.

cargo run -- scan -i %output%/Content -o ../generated_refs

:DONE
