:: Runs TPack Toolbox's `jigsaw` command.
:: Pass the name of the config file, relative to `Tools/toolbox_cfg/`,
:: minus `.toml`.
:: Run from the root dir.

echo off

set input="ExternalAssets/FactoryIn"
set output="ExternalAssets/FactoryOut"
set config="Tools/toolbox_cfg/%~1.toml"

Tools\t_pack_toolbox.exe jigsaw -i %input% -o %output% -c %config%
