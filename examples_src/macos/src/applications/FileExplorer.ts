const applicationTitles = ['File Explorer', 'Windows Explorer'];

const menuItems = [
  {
    name: 'File Explorer',
    items: [
      { name: 'About File Explorer', action: 'start ms-settings:' },
      { name: 'spacer', action: '' },
      { name: 'Preferences...', action: 'start ms-settings:system' },
      { name: 'spacer', action: '' },
      { name: 'Empty Bin', action: 'Clear-RecycleBin -Force' },
      { name: 'spacer', action: '' },
      { name: 'Hide File Explorer', action: '' },
      { name: 'Hide Others', action: '' },
      { name: 'Show All', action: '', disabled: true },
    ],
  },
  {
    name: 'File',
    items: [
      {
        name: 'New Explorer Window',
        action: 'start $HOME',
        icon: 'nf-md-apple_keyboard_command',
        key: 'N',
      },
      { name: 'New Folder', action: 'start $HOME' },
    ],
  },
  {
    name: 'Edit',
    items: [
      {
        name: 'Undo',
        action: '',
        icon: 'nf-md-apple_keyboard_command',
        key: 'Z',
      },
      {
        name: 'Redo',
        action: '',
        icon: 'nf-md-apple_keyboard_shift, nf-md-apple_keyboard_command',
        key: 'Z',
      },
      { name: 'spacer', action: '' },
      {
        name: 'Cut',
        action: '',
        icon: 'nf-md-apple_keyboard_command',
        key: 'X',
      },
      {
        name: 'Copy',
        action: '',
        icon: 'nf-md-apple_keyboard_command',
        key: 'C',
      },
      {
        name: 'Paste',
        action: '',
        icon: 'nf-md-apple_keyboard_command',
        key: 'V',
      },
      {
        name: 'Select All',
        action: '',
        icon: 'nf-md-apple_keyboard_command',
        key: 'A',
      },
      { name: 'spacer', action: '' },
      { name: 'Show Clipboard', action: 'start $HOME' },
      { name: 'spacer', action: '' },
      { name: 'Start Dictation', action: 'start $HOME' },
      { name: 'Emoji & Symbols', action: 'start $HOME' },
    ],
  },
  {
    name: 'View',
    items: [
      { name: 'as Icons', action: 'start $HOME' },
      { name: 'as List', action: 'start $HOME' },
      { name: 'as Columns', action: 'start $HOME' },
      { name: 'as Gallery', action: 'start $HOME' },
      { name: 'spacer', action: '' },
      { name: 'Use Stacks', action: 'start $HOME' },
      { name: 'Show View Options', action: 'start $HOME' },
    ],
  },
  {
    name: 'Go',
    items: [
      { name: 'Back', action: 'start $HOME' },
      { name: 'Forward', action: 'start $HOME' },
      { name: 'Enclosing Folder', action: 'start $HOME' },
      { name: 'spacer', action: '' },
      { name: 'Recents', action: 'start $HOME' },
      { name: 'Documents', action: 'start $HOME' },
      { name: 'Desktop', action: 'start $HOME' },
      { name: 'Downloads', action: 'start $HOME' },
      { name: 'Home', action: 'start $HOME' },
      { name: 'Computer', action: 'start $HOME' },
      { name: 'AirDrop', action: 'start $HOME' },
      { name: 'Network', action: 'start $HOME' },
      { name: 'iCloud Drive', action: 'start $HOME' },
      { name: 'Shared', action: 'start $HOME' },
      { name: 'Applications', action: 'start $HOME' },
      { name: 'Utilities', action: 'start $HOME' },
      { name: 'spacer', action: '' },
      {
        name: 'Go to Folder...',
        action: 'start $HOME',
        icon: 'nf-md-apple_keyboard_command',
        key: 'N',        
      },
      { name: 'Connect to Server...', action: 'start $HOME' },
    ],
  },
  {
    name: 'Window',
    items: [
      { name: 'Bring All to Front', action: 'start ms-contact-support:' },
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
