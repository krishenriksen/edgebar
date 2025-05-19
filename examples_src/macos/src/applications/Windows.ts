const menuItems = [
  {
    name: '',
    items: [
      { name: 'About This Windows', action: 'start ms-settings:' },
      { name: 'spacer', action: '' },
      {
        name: 'System Preferences...',
        action: 'start ms-settings:system'
      },
      { name: 'App Store', action: 'start ms-windows-store:' },
      { name: 'spacer', action: '' },
      {
        name: 'Force Quit...',
        action: 'Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait("%{F4}")',
        icon: 'nf-md-apple_keyboard_option',
        key: 'F4',
      },
      { name: 'spacer', action: '' },
      { name: 'Sleep', action: 'rundll32.exe powrprof.dll,SetSuspendState 0,1,0' },
      { name: 'Restart...', action: 'shutdown /r /t 0' },
      { name: 'Shut Down...', action: 'shutdown /s /t 0' },
      { name: 'spacer', action: '' },
      {
        name: 'Lock Screen',
        action: 'rundll32.exe user32.dll,LockWorkStation',
        icon: 'nf-md-apple_keyboard_command',
        key: 'L',
      },
      { name: 'Log Out...', action: 'shutdown /l' },
    ],
  },
];

export { menuItems };
