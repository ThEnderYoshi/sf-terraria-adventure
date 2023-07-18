:: Builds the pack into SansFanficTerrariaAdventureRelease
:: Run with `--check` to scan the copy.

ECHO OFF

ECHO.
ECHO "Building pack..."
ECHO.

CD Tools\pack_diagnostic
cargo run -- build -i ../.. -o ../../../SansFanficTerrariaAdventureRelease -r ../generated_refs

if "%~1"=="--check" (GOTO CHECK) ELSE GOTO DONE

:CHECK

ECHO.
ECHO "Scanning copy..."
ECHO.

cargo run -- scan -i ../../../SansFanficTerrariaAdventureRelease/Content -o ../generated_refs

:DONE
