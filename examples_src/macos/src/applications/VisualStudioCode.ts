const applicationTitles = ['Visual Studio Code'];

const menuItems = [
  {
    name: 'Code',
    items: [
      { name: 'About Visual Studio Code', action: '' },
      { name: 'Check for Updates...', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Hide Visual Studio Code', action: '' },
      { name: 'Hide Others', action: '' },
      { name: 'Show All', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Quit Visual Studio Code', action: 'Stop-Process -Name "Code' },
    ],
  },
  {
    name: 'File',
    items: [
      { name: 'New Text File', action: 'start code' },
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
      { name: 'Find in Files', action: '' },
      { name: 'Replace in Files', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Toggle Linne Comment', action: '' },
      { name: 'Toggle Block Comment', action: '' },
      { name: 'Emmet: Expand Abbreviation', action: '' },
    ],
  },
  {
    name: 'Selection',
    items: [
      { name: 'Select All', action: '' },
      { name: 'Select to Bracket', action: '' },
      { name: 'Select to Indentation', action: '' },
      { name: 'Select to Word Start', action: '' },
      { name: 'Select to Word End', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Expand Selection', action: '' },
      { name: 'Shrink Selection', action: '' },
    ],
  },
  {
    name: 'View',
    items: [
      { name: 'Command Palette', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Appearance', action: '' },
      { name: 'Editor Layout', action: '' },
      { name: 'Editor Groups', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Show Activity Bar', action: '' },
      { name: 'Show Side Bar', action: '' },
      { name: 'Show Panel', action: '' },
    ],
  },
  {
    name: 'Go',
    items: [
      { name: 'Back', action: '' },
      { name: 'Forward', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Go to File...', action: '' },
      { name: 'Go to Symbol...', action: '' },
      { name: 'Go to Definition', action: '' },
      { name: 'Go to Type Definition', action: '' },
      { name: 'Go to Implementation', action: '' },
    ],
  },
  {
    name: 'Run',
    items: [
      { name: 'Run Without Debugging', action: '' },
      { name: 'Start Debugging', action: '' },
      { name: 'Run Build Task...', action: '' },
      { name: 'Run Test', action: '' },
      { name: 'Run Task...', action: '' },
    ],
  },
  {
    name: 'Terminal',
    items: [
      { name: 'New Terminal', action: '' },
      { name: 'Split Terminal', action: '' },
      { name: 'Run Active File', action: '' },
      { name: 'Run Selected Text', action: '' },
      { name: 'Run Selected Text in Active Terminal', action: '' },
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
