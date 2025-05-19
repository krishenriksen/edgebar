const applicationTitles = ['Terminal', 'PowerShell', 'PowerShell 7 (x64)', 'PowerShell 7 (x32)', 'Command Prompt'];

const menuItems = [
  {
    name: 'Terminal',
    items: [
      { name: 'About Terminal', action: 'start pwsh' },
      { name: 'spacer', action: '' },
      { name: 'Preferences...', action: 'start pwsh' },
      { name: 'spacer', action: '' },
      { name: 'Hide Terminal', action: '' },
      { name: 'Hide Others', action: ''},
      { name: 'Show All', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Quit Terminal', action: 'Stop-Process -Name "pwsh' },
    ],
  },
  {
    name: 'Shell',
    items: [
      { name: 'Open', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Close Window', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Print', action: '' },
    ],
  },
  {
    name: 'Edit',
    items: [
      { name: 'Undo', action: '' },
      { name: 'Redo', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Cut', action: '' },
      { name: 'Copy', action: '' },
      { name: 'Paste', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Find', action: '' },
      { name: 'Replace', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Find', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Toggle Linne Comment', action: '' },
      { name: 'Toggle Block Comment', action: '' },
      { name: 'Emmet: Expand Abbreviation', action: '' },
    ],
  },
  {
    name: 'View',
    items: [
      { name: 'Show All', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Zoom In', action: '' },
      { name: 'Zoom Out', action: '' },
      { name: 'Reset Zoom', action: '' },
    ],
  },
  {
    name: 'Window',
    items: [
      { name: 'Minimize', action: '' },
      { name: 'Zoom', action: '' },
      { name: 'Bring All to Front', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Next Tab', action: '' },
      { name: 'Previous Tab', action: '' },
    ],
  },
  {
    name: 'Help',
    items: [
      { name: 'Windows Help', action: 'start ms-contact-support:' },
      { name: 'spacer', action: '' },
      {
        name: "See What's New in Windows",
        action: 'start ms-contact-support:',
      },
      {
        name: 'New to Windows? Learn the Basics',
        action: 'start ms-contact-support:',
      },
    ],
  },  
];

export { applicationTitles, menuItems };
