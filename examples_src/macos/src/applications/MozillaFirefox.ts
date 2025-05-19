const applicationTitles = ['Mozilla Firefox'];

const menuItems = [
  {
    name: 'Firefox',
    items: [
      { name: 'About Firefox', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Preferences', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Services', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Hide Firefox', action: '' },
      { name: 'Hide Others', action: '' },
      { name: 'Show All', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Quit Firefox', action: 'Stop-Process -Name "firefox"' },
    ],
  },
  {
    name: 'File',
    items: [
      { name: 'New Tab', action: 'start firefox -ArgumentList "about:newtab"' },
      { name: 'New Window', action: 'start firefox' },
      { name: 'New Private Window', action: 'start firefox -ArgumentList "-private-window"' },
      { name: 'Open File', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Close Tab', action: '' },
      { name: 'Close Window', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Save Page As...', action: '' },
      { name: 'Share', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Print', action: '' },
      { name: 'Import From Another Browser...', action: '' },
      { name: 'Work Offline', action: '' },
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
      { name: 'Delete', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Select All', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Find in Page...', action: '' },
      { name: 'Find Again', action: '' },
    ],
  },
  {
    name: 'View',
    items: [{ name: 'Enter Full Screen', action: 'Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait("{F11}")' }],
  },
  {
    name: 'History',
    items: [
      { name: 'Show All History', action: '' },
      { name: 'Clear Recent History...', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Synced Tabs', action: '' },
      { name: 'Search History', action: '' },
    ],
  },
  {
    name: 'Bookmarks',
    items: [
      { name: 'Manage Bookmarks', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Edit This Bookmark...', action: '' },
      { name: 'Search Bookmarks', action: '' },
      { name: 'Bookmark all Tabs...', action: '' },
    ],
  },
  {
    name: 'Tools',
    items: [
      { name: 'Downloads', action: '' },
      { name: 'Add-ons and Themes', action: '' },
      { name: 'Sync Now', action: '' },
      { name: 'Firefox View', action: '' },
      { name: 'spacer', action: '' },
      {
        name: 'Browser Tools',
        items: [
          { name: 'Web Developer Tools', action: '' },
          { name: 'Task Manager', action: '' },
          { name: 'Remote Debugging', action: '' },
          { name: 'Browser Console', action: '' },
          { name: 'Responsive Design Mode', action: '' },
          { name: 'Eyedropper', action: '' },
          { name: 'Page Source', action: '' },
          { name: 'Extension for Developers', action: '' },
        ],
      },
      { name: 'Page Info', action: '' },
    ],
  },
  {
    name: 'Window',
    items: [
      { name: 'Move Window to Left Side of Screen', action: '' },
      { name: 'Move Window to Right Side of Screen', action: '' },
      { name: 'Replace Tiled Window', action: '' },
      { name: 'spacer', action: '' },
      { name: 'Minimize', action: '' },
      { name: 'Zoom', action: '' },
      { name: 'Mozilla Firefox', action: '' },
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
