# Fortnite Custom Resolution Utility

## Overview

Welcome to the Fortnite Custom Resolution Utility! This command-line tool allows you to modify the screen resolution in Fortnite, enabling the use of different display formats not available in the game settings. Follow the instructions below to customize your resolution.

## Prerequisites

- This utility is designed for Windows users.

## Usage

1. Download the latest release from the [Release Section](https://github.com/nayzflux/ForniteCustomRes/releases).

2. Extract the contents of the downloaded zip file.

3. **Run the utility as Administrator**:
    - Locate and run the `FortniteCustomRes.exe` file.
    - Right-click and select "Run as Administrator" to ensure proper access.

4. Follow the on-screen prompts to customize your Fortnite resolution.

## Instructions

1. **Screen Resolution Selection:**
    - The utility will prompt you to select your current screen resolution from a list.

2. **Custom Format Selection:**
    - Choose a custom display format that suits your preference (16:9, 4:3, 5:4, 21:9, 32:9, 16:10).

3. **GameUserSettings.ini Modification:**
    - The utility will automatically update the `GameUserSettings.ini` file located at:
      ```
      %USERPROFILE%\AppData\Local\FortniteGame\Saved\Config\WindowsClient\GameUserSettings.ini
      ```
    - The selected custom resolution will be applied to the following settings:
      ```
      LastUserConfirmedResolutionSizeX
      LastUserConfirmedResolutionSizeY
      ResolutionSizeX
      ResolutionSizeY
      LastUserConfirmedDesiredScreenWidth
      LastUserConfirmedDesiredScreenHeight
      DesiredScreenWidth
      DesiredScreenHeight
      PreferredFullscreenMode
      LastConfirmedFullscreenMode
      ```

4. **Saving Changes:**
    - The modified `GameUserSettings.ini` file will be saved, and the read-only status will be restored.

## Important Note

- Ensure Fortnite is closed before running the utility.
- **Run the utility as Administrator** to ensure proper access to modify game files.
- Use this tool responsibly, and be aware that modifying game files may have consequences.

## Support

For issues or additional assistance, please [create an issue](https://github.com/nayzflux/ForniteCustomRes/issues) on the GitHub repository.

Happy gaming! 🎮