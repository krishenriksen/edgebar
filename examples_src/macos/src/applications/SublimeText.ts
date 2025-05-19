const applicationTitles = ['Sublime Text', 'Sublime Text (UNREGISTERED)'];

const menuItems = [
  {
    name: 'Sublime Text',
    items: [
      { name: 'About Sublime Text', action: 'Start-Process "sublime_text.exe"' },
      { name: 'spacer', action: '' },
      { name: 'Preferences...', action: 'Start-Process "sublime_text.exe"' },
      { name: 'spacer', action: '' },
      { name: 'Hide Sublime Text', action: '' },
      { name: 'Hide Others', action: ''},
      { name: 'Show All', action: '', disabled: true },
      { name: 'spacer', action: '' },
      { name: 'Quit Sublime Text', action: 'Stop-Process -Name "sublime_text"' },
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