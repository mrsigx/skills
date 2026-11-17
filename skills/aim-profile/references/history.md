# History sources (G0)

Read-only. Paths below were observed on one Windows machine between 2026-10-01 and 2026-10-04; games move their files between versions, so confirm each path before relying on it. Read with ordinary file reads. Never write here during G0.

## Inventory first

- Steam: every library listed in `C:\Program Files (x86)\Steam\steamapps\libraryfolders.vdf`; each library's `steamapps\appmanifest_*.acf` holds `"name"`, `"installdir"` and `"buildid"`. Libraries on other drives are common.
- Unreal Engine games usually keep user settings under `%LOCALAPPDATA%\<Project>\Saved\Config\Windows*\GameUserSettings.ini`, and logs under `...\Saved\Logs\`.
- Check whether a game is running before treating its file as current; many games write settings only on Apply or exit.

## Per-game locations

| Game | File | Aim keys | Notes |
|---|---|---|---|
| CS2 | `Steam\userdata\<id>\730\local\cfg\cs2_user_convars_0_slot0.vcfg` | `sensitivity`, `zoom_sensitivity_ratio`, `m_yaw` | `m_yaw` is the stored yaw convar; its current engine effect is not proven by the file. |
| Escape from Tarkov | `%APPDATA%\Battlestate Games\Escape from Tarkov\Settings\Control.ini` (JSON) | `MouseSensitivity`, `MouseAimingSensitivity`, `OpticSensitivity` | `Game.ini` holds `FieldOfView` (vertical). Stated by a player 2026-10-02: `OpticSensitivity` adjusts how quickly a variable scope's zoom responds, not look gain. |
| Escape from Tarkov history | `<install>\build\Logs\log_<date>_<version>\<date>_<version> application_000.log` | the `Control settings:` JSON logged at each launch | One entry per launch gives a dated series of every value; this is how a past setting and the session it changed in can be found. |
| PUBG | `%LOCALAPPDATA%\TslGame\Saved\Config\WindowsNoEditor\GameUserSettings.ini` | `SensitiveName="Normal" / "Targeting" / "Scoping" / "ScopingMagnified"` with `Sensitivity` and `LastConvertedSensitivity`; `FpsCameraFov` | The file can outlive the install; note its date. |
| WARDOGS | `%LOCALAPPDATA%\Wardogs\Saved\Config\WindowsClient\GameUserSettings.ini` | `MouseSensitivity`, `MouseADSSensitivityMultiplier`, `AimingFovSensitivityMode`, `AimingSensitivityRelativeCoefficient`, `AimingFovMode`, `FieldOfView` | `MouseSensitivity` is stored encoded, not as the menu number (see games.md). `Saved\SaveGames\EnhancedInputUserSettings.sav` held only key bindings. |
| Aim Labs | `%USERPROFILE%\AppData\LocalLow\Statespace\aimlab_tb\Users\<id>\WindowsPlayer_PlayerSettingsData.json` | `gameProfile`, `mouseSensitivityX/Y`, `lookVertFov`, `zoomProfile`, `aDSMouseSensitivityX/Y`, `zoomVertFov`, `zoomMonitorDistance`, `zoomConversion` | Shows which game profile, FOVs and ADS conversion the player trained with. |

## Lessons that generalize

- **Stored is not displayed.** A file may hold an encoded value. Decode only with a model, check it against at least two known menu/file pairs including the game's default, and still label it Estimated until measured.
- **Changed in-game is not saved.** A value the player remembers setting may never have been written (no Apply, or the game closed without saving). Report the file's value and its write time; do not assume the remembered one.
- **Ask what a key does.** Names mislead. Confirm with the player before converting a key as look gain.
- **Endpoints are not history.** Equal values in two snapshots do not prove nothing changed in between; use per-launch logs where a game provides them.
- **DPI lives outside the game.** Ask for it, or read it from the mouse vendor's software only if the player points you there.
