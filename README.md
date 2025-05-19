<div align="center">

# macOS top bar for Windows 11

[![Low system resources macOS theme for Windows 11](https://i.ytimg.com/vi/DdTJlD7R1Wg/maxresdefault.jpg)](https://youtu.be/DdTJlD7R1Wg)
</div>

## ⚙️ Installation

**Downloads for Windows 11 are available in the [latest release](https://github.com/krishenriksen/edgebar/releases)**.

## Usage

```bash
$ npm install
$ npm run build
```

Shell privileges requires the following:

`powershell`

# Requirements

## windhawk

https://github.com/ramensoftware/windhawk

### Disable grouping on the taskbar

Place ungrouped items together: **Checked**

Use window icons: **Unchecked**

### Disable Taskbar Thumbnails

Preview on hover: **Disabled**

Disable tooltips on hover: **Unchecked**

Customize the old taskbar on Windows 11: **Unchecked**

### Resource Redirect

Theme folder: **%USERPROFILE%/.edgebar/macos/assets/icons**

### Windows 11 Taskbar Styler

Advanced -> Mod settings:

```json
{
  "theme": "DockLike",
  "controlStyles[0].target": "Taskbar.TaskbarFrame#TaskbarFrame > Grid#RootGrid",
  "controlStyles[0].styles[0]": "Margin=0,1,0,3",
  "controlStyles[0].styles[1]": "CornerRadius=12",
  "controlStyles[1].target": "Grid#SystemTrayFrameGrid",
  "controlStyles[1].styles[0]": "CornerRadius=8",
  "controlStyles[0].styles[2]": "BorderThickness=1",
  "controlStyles[1].styles[1]": "BorderBrush:=<SolidColorBrush Color=\"{ThemeResource SurfaceStrokeColorDefault}\" />",
  "controlStyles[1].styles[2]": "Margin=8,1,8,3",
  "controlStyles[1].styles[3]": "BorderThickness=0",
  "controlStyles[1].styles[4]": "Background:=Transparent",
  "controlStyles[1].styles[5]": "Visibility=True",
  "controlStyles[2].target": "Taskbar.TaskListLabeledButtonPanel@RunningIndicatorStates > Rectangle#RunningIndicator",
  "controlStyles[2].styles[0]": "Width=4",
  "controlStyles[2].styles[1]": "Width@ActiveRunningIndicator=4",
  "controlStyles[2].styles[2]": "Height@ActiveRunningIndicator=3",
  "controlStyles[3].target": "Windows.UI.Xaml.Controls.TextBlock#InnerTextBlock[Text=]",
  "controlStyles[3].styles[0]": "Text=",
  "controlStyles[4].target": "SystemTray.ImageIconContent > Grid#ContainerGrid > Image",
  "controlStyles[4].styles[0]": "FontSize=14",
  "controlStyles[5].target": "SystemTray.TextIconContent > Grid#ContainerGrid > SystemTray.AdaptiveTextBlock#Base > TextBlock#InnerTextBlock",
  "controlStyles[5].styles[0]": "FontSize=14",
  "controlStyles[6].target": "SystemTray.Stack#NonActivatableStack",
  "controlStyles[6].styles[0]": "Visibility=Collapsed",
  "controlStyles[7].target": "Taskbar.ExperienceToggleButton#LaunchListButton[AutomationProperties.AutomationId=StartButton]",
  "controlStyles[7].styles[0]": "Visibility=Collapsed"
}
```

### icons

Change icon of File Explorer here (Finder.ico):

**%USERPROFILE%\AppData\Roaming\Microsoft\Internet Explorer\Quick Launch\User Pinned\TaskBar**

Icon folder: **%USERPROFILE%/.edgebar/macos/assets/icons/Extra**
