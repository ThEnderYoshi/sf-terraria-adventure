@rem Runs TPack Toolbox's `jigsaw` command.
@rem Pass the name of the config file, relative to `Tools/toolbox_cfg/`,
@rem minus `.toml`.
@rem Run from the root dir.

echo off

set input="ExternalAssets/FactoryIn"
set output="ExternalAssets/FactoryOut"
set config="Tools/toolbox_cfg/%~1.toml"

Tools\t_pack_app.exe jigsaw %input% %output% %config%
